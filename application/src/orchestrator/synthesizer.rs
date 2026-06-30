use anyhow::Result;

use crate::agent::SubAgentResult;
use crate::llm::{ChatMessage, LlmClient};

const SYNTHESIZER_PROMPT: &str = r#"You are a response synthesizer. Given the user's original query and multiple sub-agent outputs, produce a single concise answer.

Rules:
- Ultra concise (1-3 sentences max unless more detail is explicitly needed)
- No preambles
- Merge insights, don't just concatenate
- If sub-agents conflict, note the discrepancy briefly
- Professional slang ok, TLDR style"#;

pub struct Synthesizer;

impl Synthesizer {
    pub async fn synthesize(
        client: &LlmClient,
        model: &str,
        core_context: &str,
        user_query: &str,
        results: &[SubAgentResult],
    ) -> Result<String> {
        let mut agent_outputs = String::new();
        for result in results {
            agent_outputs.push_str(&format!(
                "\n--- Agent [{}] ---\n{}\n",
                result.role, result.output
            ));
        }

        let messages = vec![
            ChatMessage::system(&format!("{}\n\n{}", core_context, SYNTHESIZER_PROMPT)),
            ChatMessage::user(&format!(
                "Original query: {}\n\nSub-agent outputs:{}\n\nSynthesize a concise answer:",
                user_query, agent_outputs
            )),
        ];

        client.chat(model, &messages, 4096).await
    }
}
