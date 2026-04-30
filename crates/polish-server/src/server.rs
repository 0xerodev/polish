use crate::{LiveBus, ServerConfig};

/// Builder for a Polish HTTP server.
pub struct PolishServer {
    pub config: ServerConfig,
    pub bus: LiveBus,
}

impl PolishServer {
    pub fn new(config: ServerConfig) -> Self {
        let bus = LiveBus::new(config.live_bus_capacity);
        Self { config, bus }
    }
}

impl Default for PolishServer {
    fn default() -> Self {
        Self::new(ServerConfig::default())
    }
}
