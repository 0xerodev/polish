use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::provider::{AgentProvider, ProviderConfig, ProviderKind};

#[derive(Debug, Clone)]
pub struct RegisteredAgent {
    pub id: String,
    pub name: String,
    pub description: String,
    pub provider_kind: ProviderKind,
    pub capabilities: Vec<String>,
    pub enabled: bool,
}

pub struct AgentRegistry {
    agents: Mutex<HashMap<String, RegisteredAgent>>,
    providers: Mutex<HashMap<String, Arc<dyn AgentProvider>>>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        Self {
            agents: Mutex::new(HashMap::new()),
            providers: Mutex::new(HashMap::new()),
        }
    }

    pub fn register_agent(&self, agent: RegisteredAgent) {
        self.agents.lock().unwrap().insert(agent.id.clone(), agent);
    }

    pub fn register_provider(&self, id: &str, provider: Arc<dyn AgentProvider>) {
        self.providers.lock().unwrap().insert(id.to_string(), provider);
    }

    pub fn get_agent(&self, id: &str) -> Option<RegisteredAgent> {
        self.agents.lock().unwrap().get(id).cloned()
    }

    pub fn list_agents(&self) -> Vec<RegisteredAgent> {
        self.agents.lock().unwrap().values().cloned().collect()
    }

    pub fn get_provider(&self, id: &str) -> Option<Arc<dyn AgentProvider>> {
        self.providers.lock().unwrap().get(id).cloned()
    }

    pub fn provider_count(&self) -> usize {
        self.providers.lock().unwrap().len()
    }

    pub fn agent_count(&self) -> usize {
        self.agents.lock().unwrap().len()
    }
}

impl Default for AgentRegistry {
    fn default() -> Self { Self::new() }
}
