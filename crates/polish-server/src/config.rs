#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub live_bus_capacity: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".into(),
            port: 3000,
            live_bus_capacity: 256,
        }
    }
}

impl ServerConfig {
    pub fn new() -> Self { Self::default() }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port; self
    }

    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into(); self
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
