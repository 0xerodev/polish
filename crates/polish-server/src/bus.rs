use std::sync::Arc;
use tokio::sync::broadcast;
use polish_live::{EventKind, LiveEvent, LiveFragment};

/// tokio-broadcast-backed live event bus for axum SSE endpoints.
#[derive(Clone)]
pub struct LiveBus {
    tx: Arc<broadcast::Sender<LiveEvent>>,
}

impl LiveBus {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self { tx: Arc::new(tx) }
    }

    pub fn broadcast(&self, event: LiveEvent) {
        let _ = self.tx.send(event);
    }

    pub fn message(&self, data: impl Into<String>) {
        self.broadcast(LiveEvent::message(data));
    }

    pub fn fragment(&self, frag: LiveFragment) {
        self.broadcast(LiveEvent {
            id: None,
            kind: EventKind::Fragment,
            data: frag.to_sse_data(),
            retry_ms: None,
        });
    }

    pub fn heartbeat(&self) {
        self.broadcast(LiveEvent::heartbeat());
    }

    pub fn subscribe(&self) -> broadcast::Receiver<LiveEvent> {
        self.tx.subscribe()
    }
}
