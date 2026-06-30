use anyhow::Result;

use crate::agent::SubAgentSpec;
use crate::llm::{ChatMessage, LlmClient};

const PLANNER_PROMPT: &str = r#"You are a task decomposition planner. Given a user query, decide if it needs multiple sub-agents or can be answered directly.

If it needs decomposition, output a JSON array of sub-agent specs:
[{"role": "<agent role>", "task": "<specific task for this agent>", "model_hint": "<model_id>"}]

Model IDs available:
- claude-haiku-4-20250414 (simple/fast tasks)
- claude-sonnet-4-20250514 (standard tasks)
- claude-opus-4-20250514 (complex reasoning)

If the query is simple enough for a direct answer, output: []

Rules:
- Max 5 sub-agents
- Each agent should have a clear, focused task
- Use haiku for simple lookups, sonnet for analysis, opus for deep reasoning
- Output ONLY valid JSON, no explanation"#;

pub struct Planner;

impl Planner {
    pub async fn plan(
        client: &LlmClient,
        model: &str,
        core_context: &str,
        user_query: &str,
    ) -> Result<Vec<SubAgentSpec>> {
        let messages = vec![
            ChatMessage::system(&format!("{}\n\n{}", core_context, PLANNER_PROMPT)),
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
