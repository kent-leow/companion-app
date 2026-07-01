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

        if self.needs_web_search(&user_msg, history) {
            return self.handle_with_skill(history, "web-search").await;
        }

        if self.is_simple_query(&user_msg) {
            return self.direct_response(history).await;
        }

        let skill_names = self.skills.list();
        let plan = Planner::plan(
            &self.client,
            self.model_selector.select(Complexity::Medium),
            &self.core_context,
            &user_msg,
            &skill_names,
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

        let synthesis = Synthesizer::synthesize(
            &self.client,
            self.model_selector.select(Complexity::Medium),
            &self.core_context,
            &user_msg,
            &results,
        )
        .await?;

        Ok(synthesis)
    }

    fn needs_web_search(&self, input: &str, history: &[ChatMessage]) -> bool {
        if self.skills.get("web-search").is_none() {
            return false;
        }

        let lower = input.to_lowercase();

        // Follow-up references to prior context should NOT trigger search
        let followup_signals = [
            "you mentioned", "you said", "what about", "which one",
            "tell me more", "elaborate", "explain", "why",
        ];
        if followup_signals.iter().any(|s| lower.contains(s)) {
            return false;
        }

        // Only trigger on first user message or when explicitly asking for fresh info
        let has_prior_context = history
            .iter()
            .filter(|m| m.role == crate::llm::Role::Assistant)
            .count()
            > 0;

        let strong_realtime_signals = [
            "latest", "today", "current", "right now", "this week",
            "this month", "yesterday", "tonight", "happening",
            "weather", "price", "stock", "election", "live",
        ];

        let weak_realtime_signals = [
            "score", "result", "news", "update", "match", "game",
        ];

        // Strong signals always trigger search
        if strong_realtime_signals.iter().any(|s| lower.contains(s)) {
            return true;
        }

        // Weak signals only trigger on first query (no prior context)
        if !has_prior_context && weak_realtime_signals.iter().any(|s| lower.contains(s)) {
            return true;
        }

        false
    }

    fn is_simple_query(&self, input: &str) -> bool {
        let word_count = input.split_whitespace().count();
        word_count <= 10 && !input.contains("and") && !input.contains("then")
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

        let search_query = self.extract_search_query(&user_msg).await?;
        let raw_results =
            crate::skills::executor::SkillExecutor::execute(skill, &search_query)
                .unwrap_or_else(|e| format!("ERROR: search failed: {}", e));

        let has_results = !raw_results.is_empty()
            && !raw_results.contains("ERROR")
            && !raw_results.contains("unavailable");

        // Build messages preserving full conversation history
        let mut messages: Vec<ChatMessage> = Vec::new();

        let system_prompt = if has_results {
            format!(
                "{}\n\n{}\n\nWeb search results for context:\n{}",
                &self.core_context, &skill.prompt, raw_results
            )
        } else {
            format!(
                "{}\n\nWeb search was attempted but failed (network restriction or rate limit). Acknowledge this limitation, provide what you know (with caveat it may be outdated), and suggest where to check for live info.",
                &self.core_context
            )
        };

        messages.push(ChatMessage::system(&system_prompt));

        // Include conversation history (skip the original system message)
        for msg in history.iter() {
            if msg.role != crate::llm::Role::System {
                messages.push(msg.clone());
            }
        }

        let model = self.model_selector.select(Complexity::Medium);
        self.client.chat(model, &messages, 2048).await
    }

    async fn extract_search_query(&self, user_query: &str) -> Result<String> {
        let messages = vec![
            ChatMessage::system(
                "Extract the best web search query from the user's question. Output ONLY the search query, nothing else.",
            ),
            ChatMessage::user(user_query),
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
