use crate::config::EnvConfig;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Complexity {
    Low,
    Medium,
    High,
}

pub struct ModelSelector {
    haiku: String,
    sonnet: String,
    opus: String,
}

impl ModelSelector {
    pub fn from_config(config: &EnvConfig) -> Self {
        Self {
            haiku: std::env::var("ANTHROPIC_DEFAULT_HAIKU_MODEL")
                .unwrap_or_else(|_| "anthropic.claude-haiku-4-5-20251001-v1:0".to_string()),
            sonnet: config.default_model.clone(),
            opus: std::env::var("ANTHROPIC_DEFAULT_OPUS_MODEL")
                .unwrap_or_else(|_| "anthropic.claude-opus-4-6-v1".to_string()),
        }
    }

    pub fn select(&self, complexity: Complexity) -> &str {
        match complexity {
            Complexity::Low => &self.haiku,
            Complexity::Medium => &self.sonnet,
            Complexity::High => &self.opus,
        }
    }
}
