use std::time::Duration;
use tokio::time::timeout;

use crate::agent::{PromptBuilder, SubAgentResult, SubAgentSpec};
use crate::llm::LlmClient;

const AGENT_TIMEOUT: Duration = Duration::from_secs(30);

pub struct AgentPool {
    client: LlmClient,
    core_context: String,
}

impl AgentPool {
    pub fn new(client: LlmClient, core_context: String) -> Self {
        Self {
            client,
            core_context,
        }
    }

    pub async fn run_agents(&self, specs: Vec<SubAgentSpec>) -> Vec<SubAgentResult> {
        let mut handles = Vec::new();

        for spec in specs {
            let client = self.client.clone();
            let core = self.core_context.clone();

            let handle = tokio::spawn(async move {
                let messages = PromptBuilder::build(&core, &spec.role, &spec.task);
                let model = &spec.model_hint;

                let result = timeout(AGENT_TIMEOUT, client.chat(model, &messages, 2048)).await;

                match result {
                    Ok(Ok(output)) => Some(SubAgentResult {
                        role: spec.role,
                        output,
                    }),
                    Ok(Err(e)) => {
                        eprintln!("[agent:{}] error: {}", spec.role, e);
                        None
                    }
                    Err(_) => {
                        eprintln!("[agent:{}] timeout", spec.role);
                        None
                    }
                }
            });

            handles.push(handle);
        }

        let mut results = Vec::new();
        for handle in handles {
            if let Ok(Some(result)) = handle.await {
                results.push(result);
            }
        }
        results
    }
}

impl Clone for AgentPool {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            core_context: self.core_context.clone(),
        }
    }
}
