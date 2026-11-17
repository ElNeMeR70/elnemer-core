use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub agent: String,
    pub role: String,
    pub status: String,
    pub output: String,
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub name: String,
    pub role: String,
    pub instructions: String,
    memory: Vec<String>,
}

impl Agent {
    pub fn new(name: impl Into<String>, role: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            role: role.into(),
            instructions: String::new(),
            memory: Vec::new(),
        }
    }

    pub fn with_instructions(mut self, inst: impl Into<String>) -> Self {
        self.instructions = inst.into();
        self
    }

    pub fn run(&mut self, task: &str) -> TaskResult {
        self.memory.push(task.to_string());
        TaskResult {
            agent: self.name.clone(),
            role: self.role.clone(),
            status: "completed".to_string(),
            output: format!("Agent '{}' executed task: '{}' under role '{}'", self.name, task, self.role),
        }
    }
}

#[derive(Default)]
pub struct Swarm {
    pub name: String,
    agents: Vec<Agent>,
}

impl Swarm {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            agents: Vec::new(),
        }
    }

    pub fn add_agent(&mut self, agent: Agent) {
        self.agents.push(agent);
    }

    pub fn broadcast(&mut self, task: &str) -> Vec<TaskResult> {
        self.agents.iter_mut().map(|a| a.run(task)).collect()
    }
}
