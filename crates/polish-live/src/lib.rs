use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub mod event;
pub mod stream;
pub mod fragment;
pub mod channel;

pub use event::{LiveEvent, EventKind};
pub use stream::{SseStream, SseEncoder};
pub use fragment::{LiveFragment, FragmentKind};
pub use channel::{Channel, ChannelRegistry};

#[derive(Clone)]
pub struct LiveConfig {
    pub heartbeat_interval_ms: u64,
    pub max_clients_per_channel: usize,
    pub reconnect_timeout_ms: u64,
}

impl Default for LiveConfig {
    fn default() -> Self {
        Self {
            heartbeat_interval_ms: 15_000,
            max_clients_per_channel: 1000,
            reconnect_timeout_ms: 5_000,
        }
    }
}

pub type ClientId = u64;

#[derive(Clone)]
pub struct LiveRuntime {
    pub config: LiveConfig,
    pub registry: Arc<ChannelRegistry>,
}

impl LiveRuntime {
    pub fn new(config: LiveConfig) -> Self {
        Self {
            registry: Arc::new(ChannelRegistry::new(config.max_clients_per_channel)),
            config,
        }
    }

    pub fn broadcast(&self, channel: &str, event: LiveEvent) -> usize {
        self.registry.broadcast(channel, event)
    }

    pub fn push_fragment(&self, channel: &str, frag: LiveFragment) -> usize {
        let event = LiveEvent {
            id: None,
            kind: EventKind::Fragment,
            data: frag.to_sse_data(),
            retry_ms: None,
        };
        self.registry.broadcast(channel, event)
    }
}
