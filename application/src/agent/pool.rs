use std::time::Duration;
use tokio::time::timeout;

use crate::agent::{PromptBuilder, SubAgentResult, SubAgentSpec};
use crate::llm::{ChatMessage, LlmClient};
use crate::skills::executor::SkillExecutor;
use crate::skills::loader::SkillRegistry;

const AGENT_TIMEOUT: Duration = Duration::from_secs(60);

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
                    let preflight = SkillExecutor::execute(&skill, &spec.task)
                        .unwrap_or_default();

                    let system_prompt = format!(
                        "{}\n\n## Skill: {}\n\n{}\n\n## Preflight Output\n{}\n\n\
                        ## CRITICAL INSTRUCTIONS\n\
                        You CANNOT execute commands. You do NOT have shell access.\n\
                        Based on the skill instructions above, tell the user:\n\
                        1. What the skill can do for their request\n\
                        2. What credentials/setup are needed (based on preflight output)\n\
                        3. The exact commands they would need to run\n\
                        NEVER pretend you executed a command or fabricate API responses.\n\
                        NEVER invent data you did not receive. If you lack information, say so.",
                        core, skill.name, skill.prompt, preflight
                    );
                    let messages = vec![
                        ChatMessage::system(&system_prompt),
                        ChatMessage::user(&spec.task),
                    ];
                    let model = &spec.model_hint;
                    let result =
                        timeout(AGENT_TIMEOUT, client.chat(model, &messages, 4096)).await;
                    match result {
                        Ok(Ok(output)) => Some(SubAgentResult {
                            role: spec.role,
                            output,
                        }),
                        Ok(Err(e)) => {
                            eprintln!("[agent:{}] skill-llm error: {}", spec.role, e);
                            None
                        }
                        Err(_) => {
                            eprintln!("[agent:{}] skill-llm timeout", spec.role);
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
