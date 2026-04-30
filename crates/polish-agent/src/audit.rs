use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct AgentAuditEntry {
    pub ts: u64,
    pub task_id: String,
    pub agent_id: String,
    pub action: String,
    pub outcome: String,
    pub detail: Option<String>,
}

impl AgentAuditEntry {
    pub fn new(task_id: impl Into<String>, agent_id: impl Into<String>, action: impl Into<String>, outcome: impl Into<String>) -> Self {
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        Self {
            ts,
            task_id: task_id.into(),
            agent_id: agent_id.into(),
            action: action.into(),
            outcome: outcome.into(),
            detail: None,
        }
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn to_json(&self) -> String {
        let detail = match &self.detail {
            Some(d) => format!(r#","detail":"{}""#, d.replace('"', "\\\"")),
            None => String::new(),
        };
        format!(
            r#"{{"ts":{},"task_id":"{}","agent_id":"{}","action":"{}","outcome":"{}"{}}}"#,
            self.ts, self.task_id, self.agent_id, self.action, self.outcome, detail
        )
    }
}

pub struct AgentAuditLog {
    entries: Vec<AgentAuditEntry>,
    max_entries: usize,
}

impl AgentAuditLog {
    pub fn new(max_entries: usize) -> Self {
        Self { entries: Vec::new(), max_entries }
    }

    pub fn log(&mut self, entry: AgentAuditEntry) {
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0);
        }
        self.entries.push(entry);
    }

    pub fn entries(&self) -> &[AgentAuditEntry] {
        &self.entries
    }

    pub fn entries_for_task(&self, task_id: &str) -> Vec<&AgentAuditEntry> {
        self.entries.iter().filter(|e| e.task_id == task_id).collect()
    }

    pub fn to_jsonl(&self) -> String {
        self.entries.iter().map(|e| e.to_json()).collect::<Vec<_>>().join("\n")
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
