pub mod planner;
pub mod synthesizer;

pub use planner::Planner;
pub use synthesizer::Synthesizer;

use anyhow::Result;

use crate::agent::AgentPool;
use crate::llm::{ChatMessage, LlmClient, ModelSelector, Complexity};
use crate::skills::loader::SkillRegistry;

pub struct Orchestrator {
    client: LlmClient,
    model_selector: ModelSelector,
    agent_pool: AgentPool,
    core_context: String,
    skills: SkillRegistry,
}

impl Orchestrator {
    pub fn new(
        client: LlmClient,
        model_selector: ModelSelector,
        core_context: String,
        skills: SkillRegistry,
    ) -> Self {
        let agent_pool = AgentPool::new(client.clone(), core_context.clone());
        Self {
            client,
            model_selector,
            agent_pool,
            core_context,
            skills,
        }
    }

    pub async fn handle(&self, history: &[ChatMessage]) -> Result<String> {
        let user_msg = history
            .iter()
            .rev()
            .find(|m| m.role == crate::llm::Role::User)
            .map(|m| m.content.as_str().unwrap_or_default().to_string())
            .unwrap_or_default();

        let route = self.route(history).await?;

        match route.as_str() {
            "SEARCH" => self.handle_with_skill(history, "web-search").await,
            "DECOMPOSE" => {
                let skill_descs = self.skills.list_with_descriptions();
                let plan = Planner::plan(
                    &self.client,
                    self.model_selector.select(Complexity::Medium),
                    &self.core_context,
                    &user_msg,
                    &skill_descs,
                )
                .await?;

                if plan.is_empty() {
                    return self.direct_response(history).await;
                }

                let results = self
                    .agent_pool
                    .run_agents_with_skills(plan, &self.skills)
                    .await;

                if results.is_empty() {
                    return self.direct_response(history).await;
                }

                Synthesizer::synthesize(
                    &self.client,
                    self.model_selector.select(Complexity::Medium),
                    &self.core_context,
                    &user_msg,
                    &results,
                )
                .await
            }
            _ => self.direct_response(history).await,
        }
    }

    async fn route(&self, history: &[ChatMessage]) -> Result<String> {
        let has_search = self.skills.get("web-search").is_some();

        let mut context = String::new();
        for msg in history.iter() {
            if msg.role == crate::llm::Role::System {
                continue;
            }
            let role = if msg.role == crate::llm::Role::User { "User" } else { "Assistant" };
            let content = msg.content.as_str().unwrap_or_default();
            context.push_str(&format!("{}: {}\n", role, content));
        }

        let user_msg = history
            .iter()
            .rev()
            .find(|m| m.role == crate::llm::Role::User)
            .and_then(|m| m.content.as_str())
            .unwrap_or_default();
        let lower = user_msg.to_lowercase();

        let skill_patterns: &[&str] = &[
            "gitlab",
            "sgts.gitlab-dedicated.com",
            "merge_request",
            "merge request",
            "figma.com",
            "figma design",
            "jira",
            "atlassian.net",
            "vulnerability",
            "vulnerabilities",
            "security scan",
        ];
        if skill_patterns.iter().any(|p| lower.contains(p)) {
            return Ok("DECOMPOSE".to_string());
        }

        let search_option = if has_search {
            "- SEARCH — the user asks about real-time info, recent events, news, scores, prices, \
             weather, or anything that requires up-to-date data beyond your training cutoff. \
             Also use when the user explicitly asks you to search, look up, or verify something online. \
             When in doubt about whether info is current, prefer SEARCH."
        } else {
            ""
        };

        let prompt = format!(
            "You are a routing brain. Given the conversation, decide the best action.\n\
             Output ONLY one word — the route.\n\n\
             Options:\n\
             {}\n\
             - DECOMPOSE — the query is complex and needs multiple sub-agents (multi-step research, \
             comparisons, analysis requiring multiple perspectives)\n\
             - DIRECT — you can answer from general knowledge confidently, or the user is having \
             a casual conversation\n\n\
             Rules:\n\
             - If the user says \"search\", \"look up\", \"google it\", \"find\", \"check\" — always SEARCH\n\
             - If the topic involves events, releases, updates, versions, patches, tournaments, \
             scores, prices, or anything time-sensitive — always SEARCH\n\
             - If the assistant previously said it couldn't search or gave uncertain info, \
             and the user pushes back — SEARCH\n\
             - Never output anything other than SEARCH, DECOMPOSE, or DIRECT",
            search_option
        );

        let messages = vec![
            ChatMessage::system(&prompt),
            ChatMessage::user(&context),
        ];

        let model = self.model_selector.select(Complexity::Low);
        let response = self.client.chat(model, &messages, 16).await?;
        let route = response.trim().to_uppercase();

        if route == "SEARCH" && has_search {
            Ok("SEARCH".to_string())
        } else if route == "DECOMPOSE" {
            Ok("DECOMPOSE".to_string())
        } else {
            Ok("DIRECT".to_string())
        }
    }

    async fn handle_with_skill(&self, history: &[ChatMessage], skill_name: &str) -> Result<String> {
        let skill = self
            .skills
            .get(skill_name)
            .ok_or_else(|| anyhow::anyhow!("skill '{}' not found", skill_name))?;

        let user_msg = history
            .iter()
            .rev()
            .find(|m| m.role == crate::llm::Role::User)
            .map(|m| m.content.as_str().unwrap_or_default().to_string())
            .unwrap_or_default();

        let search_query = self.extract_search_query(history).await?;
        let raw_results =
            crate::skills::executor::SkillExecutor::execute(skill, &search_query)
                .unwrap_or_else(|e| format!("ERROR: search failed: {}", e));

        let has_results = !raw_results.is_empty()
            && !raw_results.contains("ERROR")
            && !raw_results.contains("unavailable");

        if !has_results {
            return self.search_failed_response(history).await;
        }

        let urls_to_fetch = self.assess_search_results(&user_msg, &raw_results).await?;

        let enriched_results = if urls_to_fetch.is_empty() {
            raw_results.clone()
        } else {
            let fetched = self.fetch_urls(&urls_to_fetch).await;
            format!("{}\n\n--- Fetched Page Content ---\n{}", raw_results, fetched)
        };

        let system_prompt = format!(
            "{}\n\n{}\n\nWeb search results for context:\n{}",
            &self.core_context, &skill.prompt, enriched_results
        );

        let mut messages: Vec<ChatMessage> = Vec::new();
        messages.push(ChatMessage::system(&system_prompt));

        for msg in history.iter() {
            if msg.role != crate::llm::Role::System {
                messages.push(msg.clone());
            }
        }

        let model = self.model_selector.select(Complexity::Medium);
        self.client.chat(model, &messages, 2048).await
    }

    async fn assess_search_results(&self, user_query: &str, results: &str) -> Result<Vec<String>> {
        let snippet_chars: usize = results
            .lines()
            .filter(|l| l.starts_with("SNIPPET:"))
            .map(|l| l.len())
            .sum();
        if snippet_chars > 400 {
            return Ok(Vec::new());
        }

        let messages = vec![
            ChatMessage::system(
                "You assess whether web search snippets are sufficient to answer a question.\n\
                 If the snippets contain enough information, respond with exactly: SUFFICIENT\n\
                 If more detail is needed, respond with up to 3 URLs from the results that would \
                 be most relevant, one per line, prefixed with FETCH:\n\
                 Example:\nFETCH: https://example.com/article\nFETCH: https://other.com/page\n\
                 Output ONLY 'SUFFICIENT' or 'FETCH:' lines, nothing else.",
            ),
            ChatMessage::user(&format!(
                "User question: {}\n\nSearch results:\n{}",
                user_query, results
            )),
        ];

        let model = self.model_selector.select(Complexity::Low);
        let response = self.client.chat(model, &messages, 256).await?;

        if response.trim() == "SUFFICIENT" {
            return Ok(Vec::new());
        }

        let urls: Vec<String> = response
            .lines()
            .filter_map(|line| line.strip_prefix("FETCH:"))
            .map(|url| url.trim().to_string())
            .filter(|url| url.starts_with("http"))
            .take(3)
            .collect();

        Ok(urls)
    }

    async fn fetch_urls(&self, urls: &[String]) -> String {
        let fetch_skill = match self.skills.get("web-fetch") {
            Some(s) => s.clone(),
            None => return String::from("[web-fetch skill not available]"),
        };

        let mut handles = Vec::new();
        for url in urls.iter().take(3) {
            let skill = fetch_skill.clone();
            let url = url.clone();
            let handle = tokio::spawn(async move {
                match crate::skills::executor::SkillExecutor::execute(&skill, &url) {
                    Ok(content) => format!("--- {} ---\n{}\n", url, content),
                    Err(e) => format!("--- {} ---\n[fetch failed: {}]\n", url, e),
                }
            });
            handles.push(handle);
        }

        let mut combined = String::new();
        for handle in handles {
            if let Ok(result) = handle.await {
                combined.push_str(&result);
            }
        }
        combined
    }

    async fn search_failed_response(&self, history: &[ChatMessage]) -> Result<String> {
        let system_prompt = format!(
            "{}\n\nWeb search was attempted but failed (network restriction or rate limit). \
             Acknowledge this limitation, provide what you know (with caveat it may be outdated), \
             and suggest where to check for live info.",
            &self.core_context
        );

        let mut messages: Vec<ChatMessage> = Vec::new();
        messages.push(ChatMessage::system(&system_prompt));
        for msg in history.iter() {
            if msg.role != crate::llm::Role::System {
                messages.push(msg.clone());
            }
        }

        let model = self.model_selector.select(Complexity::Medium);
        self.client.chat(model, &messages, 2048).await
    }

    async fn extract_search_query(&self, history: &[ChatMessage]) -> Result<String> {
        let mut context = String::new();
        for msg in history.iter() {
            if msg.role == crate::llm::Role::System {
                continue;
            }
            let role_label = if msg.role == crate::llm::Role::User { "User" } else { "Assistant" };
            let content = msg.content.as_str().unwrap_or_default();
            context.push_str(&format!("{}: {}\n", role_label, content));
        }

        let messages = vec![
            ChatMessage::system(
                "Given the conversation, extract the best web search query to answer the user's latest request. \
                 Use context from earlier messages to understand what topic they want searched. \
                 Output ONLY the search query, nothing else.",
            ),
            ChatMessage::user(&context),
        ];
        let model = self.model_selector.select(Complexity::Low);
        let query = self.client.chat(model, &messages, 64).await?;
        Ok(query.trim().trim_matches('"').to_string())
    }

    async fn direct_response(&self, history: &[ChatMessage]) -> Result<String> {
        let model = self.model_selector.select(Complexity::Medium);
        self.client.chat(model, history, 4096).await
    }
}
