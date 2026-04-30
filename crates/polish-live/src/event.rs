use std::fmt::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventKind {
    Message,
    Fragment,
    Heartbeat,
    StateSync,
    Custom(String),
}

impl EventKind {
    pub fn as_str(&self) -> &str {
        match self {
            EventKind::Message => "message",
            EventKind::Fragment => "fragment",
            EventKind::Heartbeat => "heartbeat",
            EventKind::StateSync => "state_sync",
            EventKind::Custom(s) => s.as_str(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LiveEvent {
    pub id: Option<String>,
    pub kind: EventKind,
    pub data: String,
    pub retry_ms: Option<u64>,
}

impl LiveEvent {
    pub fn message(data: impl Into<String>) -> Self {
        Self { id: None, kind: EventKind::Message, data: data.into(), retry_ms: None }
    }

    pub fn heartbeat() -> Self {
        Self { id: None, kind: EventKind::Heartbeat, data: "ping".into(), retry_ms: None }
    }

    pub fn state_sync(json: impl Into<String>) -> Self {
        Self { id: None, kind: EventKind::StateSync, data: json.into(), retry_ms: None }
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_retry(mut self, ms: u64) -> Self {
        self.retry_ms = Some(ms);
        self
    }

    pub fn to_sse_bytes(&self) -> Vec<u8> {
        let mut buf = String::new();
        if let Some(id) = &self.id {
            let _ = writeln!(buf, "id: {id}");
        }
        if self.kind != EventKind::Message {
            let _ = writeln!(buf, "event: {}", self.kind.as_str());
        }
        if let Some(ms) = self.retry_ms {
            let _ = writeln!(buf, "retry: {ms}");
        }
        for line in self.data.lines() {
            let _ = writeln!(buf, "data: {line}");
        }
        buf.push('\n');
        buf.into_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sse_format_message() {
        let ev = LiveEvent::message("hello world");
        let bytes = ev.to_sse_bytes();
        let s = String::from_utf8(bytes).unwrap();
        assert!(s.contains("data: hello world\n"));
        assert!(s.ends_with("\n\n"));
    }

    #[test]
    fn sse_format_custom_event() {
        let ev = LiveEvent { id: Some("42".into()), kind: EventKind::Fragment, data: "{}".into(), retry_ms: None };
        let bytes = ev.to_sse_bytes();
        let s = String::from_utf8(bytes).unwrap();
        assert!(s.contains("id: 42\n"));
        assert!(s.contains("event: fragment\n"));
    }
}
