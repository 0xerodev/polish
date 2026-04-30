use crate::event::LiveEvent;

pub struct SseStream {
    events: Vec<LiveEvent>,
}

impl SseStream {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn push(&mut self, event: LiveEvent) {
        self.events.push(event);
    }

    pub fn drain_bytes(&mut self) -> Vec<u8> {
        let mut out = Vec::new();
        for ev in self.events.drain(..) {
            out.extend_from_slice(&ev.to_sse_bytes());
        }
        out
    }
}

impl Default for SseStream {
    fn default() -> Self { Self::new() }
}

pub struct SseEncoder;

impl SseEncoder {
    pub fn encode(event: &LiveEvent) -> Vec<u8> {
        event.to_sse_bytes()
    }

    pub fn comment(text: &str) -> Vec<u8> {
        format!(": {text}\n\n").into_bytes()
    }

    pub fn retry(ms: u64) -> Vec<u8> {
        format!("retry: {ms}\n\n").into_bytes()
    }
}
