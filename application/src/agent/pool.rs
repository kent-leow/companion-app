use std::time::Duration;
use tokio::time::timeout;

use crate::agent::{PromptBuilder, SubAgentResult, SubAgentSpec};
use crate::llm::LlmClient;
use crate::skills::executor::SkillExecutor;
use crate::skills::loader::SkillRegistry;

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

    pub async fn run_agents_with_skills(
        &self,
        specs: Vec<SubAgentSpec>,
        skills: &SkillRegistry,
    ) -> Vec<SubAgentResult> {
        let mut handles = Vec::new();

        for spec in specs {
            let client = self.client.clone();
            let core = self.core_context.clone();
            let skill_data = spec.skill.as_ref().and_then(|name| {
                skills.get(name).map(|s| s.clone())
            });

            let handle = tokio::spawn(async move {
                if let Some(skill) = skill_data {
                    match SkillExecutor::execute(&skill, &spec.task) {
                        Ok(output) => Some(SubAgentResult {
                            role: spec.role,
                            output,
                        }),
                        Err(e) => {
                            eprintln!("[agent:{}] skill error: {}", spec.role, e);
                            None
                        }
                    }
                } else {
                    let messages = PromptBuilder::build(&core, &spec.role, &spec.task);
                    let model = &spec.model_hint;
                    let result =
                        timeout(AGENT_TIMEOUT, client.chat(model, &messages, 2048)).await;
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
