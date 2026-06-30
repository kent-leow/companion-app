use anyhow::Result;

use crate::agent::SubAgentSpec;
use crate::llm::{ChatMessage, LlmClient};

const PLANNER_PROMPT_PREFIX: &str = r#"You are a task decomposition planner. Given a user query, decide if it needs multiple sub-agents or can be answered directly.

If it needs decomposition, output a JSON array of sub-agent specs:
[{"role": "<agent role>", "task": "<specific task for this agent>", "model_hint": "<model_id>", "skill": "<skill_name or null>"}]

Model IDs available:
- bedrock.claude-haiku-4-5 (simple/fast tasks)
- bedrock.claude-sonnet-4-6 (standard tasks)
- bedrock.claude-opus-4-6 (complex reasoning)

If the query is simple enough for a direct answer, output: []

Rules:
- Max 5 sub-agents
- Each agent should have a clear, focused task
- Use haiku for simple lookups, sonnet for analysis, opus for deep reasoning
- If a task can be handled by a skill, set "skill" to the skill name (the agent will execute it)
- For real-time info (news, scores, weather, prices), USE the web-search skill
- Output ONLY valid JSON, no explanation"#;

pub struct Planner;

impl Planner {
    pub async fn plan(
        client: &LlmClient,
        model: &str,
        core_context: &str,
        user_query: &str,
        available_skills: &[&str],
    ) -> Result<Vec<SubAgentSpec>> {
        let skills_section = if available_skills.is_empty() {
            String::new()
        } else {
            format!(
                "\n\nAvailable skills (use these for tasks they can handle):\n{}",
                available_skills
                    .iter()
                    .map(|s| format!("- {}", s))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        };

        let prompt = format!("{}{}", PLANNER_PROMPT_PREFIX, skills_section);
        let messages = vec![
            ChatMessage::system(&format!("{}\n\n{}", core_context, prompt)),
            ChatMessage::user(user_query),
        ];

        let response = client.chat(model, &messages, 1024).await?;

        let json_str = extract_json(&response);
        let specs: Vec<SubAgentSpec> = serde_json::from_str(json_str).unwrap_or_default();

        Ok(specs)
    }
}

fn extract_json(text: &str) -> &str {
    let trimmed = text.trim();
    if trimmed.starts_with('[') {
        if let Some(end) = trimmed.rfind(']') {
            return &trimmed[..=end];
        }
    }
    if let Some(start) = trimmed.find('[') {
        if let Some(end) = trimmed.rfind(']') {
            return &trimmed[start..=end];
        }
    }
    "[]"
}
