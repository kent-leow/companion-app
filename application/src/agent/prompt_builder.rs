use crate::llm::ChatMessage;

pub struct PromptBuilder;

impl PromptBuilder {
    pub fn build(core_context: &str, role: &str, task: &str) -> Vec<ChatMessage> {
        let system_prompt = format!(
            "{}\n\n---\nYour role: {}\nRespond concisely. No preambles.",
            core_context, role
        );

        vec![
            ChatMessage::system(&system_prompt),
            ChatMessage::user(task),
        ]
    }
}
