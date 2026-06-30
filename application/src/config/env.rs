use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct EnvConfig {
    pub base_url: String,
    pub auth_token: String,
    pub default_model: String,
}

impl EnvConfig {
    pub fn load() -> Result<Self> {
        let base_url = std::env::var("ANTHROPIC_BASE_URL")
            .context("ANTHROPIC_BASE_URL not set")?;

        let auth_token = std::env::var("ANTHROPIC_AUTH_TOKEN")
            .context("ANTHROPIC_AUTH_TOKEN not set")?;

        let default_model = std::env::var("ANTHROPIC_DEFAULT_SONNET_MODEL")
            .unwrap_or_else(|_| "anthropic.claude-sonnet-4-5-20250929-v1:0".to_string());

        Ok(Self {
            base_url,
            auth_token,
            default_model,
        })
    }
}
