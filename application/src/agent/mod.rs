pub mod pool;
pub mod prompt_builder;

pub use pool::AgentPool;
pub use prompt_builder::PromptBuilder;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAgentSpec {
    pub role: String,
    pub task: String,
    pub model_hint: String,
    #[serde(default)]
    pub skill: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SubAgentResult {
    pub role: String,
    pub output: String,
}
