use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl TaskStatus {
    pub fn as_str(&self) -> &str {
        match self {
            TaskStatus::Pending => "pending",
            TaskStatus::Running => "running",
            TaskStatus::Completed => "completed",
            TaskStatus::Failed => "failed",
            TaskStatus::Cancelled => "cancelled",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AgentTask {
    pub id: String,
    pub agent_id: String,
    pub prompt: String,
    pub context: Vec<(String, String)>,
    pub status: TaskStatus,
    pub created_at: u64,
}

impl AgentTask {
    pub fn new(agent_id: impl Into<String>, prompt: impl Into<String>) -> Self {
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        Self {
            id: format!("task_{ts}"),
            agent_id: agent_id.into(),
            prompt: prompt.into(),
            context: Vec::new(),
            status: TaskStatus::Pending,
            created_at: ts,
        }
    }

    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context.push((key.into(), value.into()));
        self
    }
}

#[derive(Debug, Clone)]
pub struct TaskResult {
    pub task_id: String,
    pub status: TaskStatus,
    pub output: Option<String>,
    pub error: Option<String>,
    pub tokens_used: Option<u32>,
    pub duration_ms: u64,
}

impl TaskResult {
    pub fn success(task_id: impl Into<String>, output: impl Into<String>) -> Self {
        Self {
            task_id: task_id.into(),
            status: TaskStatus::Completed,
            output: Some(output.into()),
            error: None,
            tokens_used: None,
            duration_ms: 0,
        }
    }

    pub fn failure(task_id: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            task_id: task_id.into(),
            status: TaskStatus::Failed,
            output: None,
            error: Some(error.into()),
            tokens_used: None,
            duration_ms: 0,
        }
    }

    pub fn is_ok(&self) -> bool {
        self.status == TaskStatus::Completed
    }
}
