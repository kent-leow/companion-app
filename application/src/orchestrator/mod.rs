pub mod planner;
pub mod synthesizer;

pub use planner::Planner;
pub use synthesizer::Synthesizer;

use anyhow::Result;

use crate::agent::{AgentPool, SubAgentSpec};
use crate::llm::{ChatMessage, LlmClient, ModelSelector, Complexity};

pub struct Orchestrator {
    client: LlmClient,
    model_selector: ModelSelector,
    agent_pool: AgentPool,
    core_context: String,
}

impl Orchestrator {
    pub fn new(client: LlmClient, model_selector: ModelSelector, core_context: String) -> Self {
        let agent_pool = AgentPool::new(client.clone(), core_context.clone());
        Self {
            client,
            model_selector,
            agent_pool,
            core_context,
        }
    }

    pub async fn handle(&self, history: &[ChatMessage]) -> Result<String> {
        let user_msg = history
            .iter()
            .rev()
            .find(|m| m.role == crate::llm::Role::User)
            .map(|m| m.content.as_str().unwrap_or_default().to_string())
            .unwrap_or_default();

        if self.is_simple_query(&user_msg) {
            return self.direct_response(history).await;
        }

        let plan = Planner::plan(&self.client, self.model_selector.select(Complexity::Medium), &self.core_context, &user_msg).await?;

        if plan.is_empty() {
            return self.direct_response(history).await;
        }

        let results = self.agent_pool.run_agents(plan).await;

        if results.is_empty() {
            return self.direct_response(history).await;
        }

        let synthesis = Synthesizer::synthesize(
            &self.client,
            self.model_selector.select(Complexity::Medium),
            &self.core_context,
            &user_msg,
            &results,
        )
        .await?;

        Ok(synthesis)
    }

    fn is_simple_query(&self, input: &str) -> bool {
        let word_count = input.split_whitespace().count();
        word_count <= 10 && !input.contains("and") && !input.contains("then")
    }

    async fn direct_response(&self, history: &[ChatMessage]) -> Result<String> {
        let model = self.model_selector.select(Complexity::Medium);
        self.client.chat(model, history, 4096).await
    }
}
