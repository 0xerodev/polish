#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderKind {
    Native,
    OpenAI,
    Anthropic,
    Local,
    Mcp,
}

impl ProviderKind {
    pub fn as_str(&self) -> &str {
        match self {
            ProviderKind::Native => "native",
            ProviderKind::OpenAI => "openai",
            ProviderKind::Anthropic => "anthropic",
            ProviderKind::Local => "local",
            ProviderKind::Mcp => "mcp",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub kind: ProviderKind,
    pub model: Option<String>,
    pub endpoint: Option<String>,
    pub api_key_env: Option<String>,
    pub max_tokens: u32,
    pub temperature: f32,
}

impl ProviderConfig {
    pub fn native() -> Self {
        Self { kind: ProviderKind::Native, model: None, endpoint: None, api_key_env: None, max_tokens: 2048, temperature: 0.0 }
    }

    pub fn anthropic(model: impl Into<String>) -> Self {
        Self {
            kind: ProviderKind::Anthropic,
            model: Some(model.into()),
            endpoint: Some("https://api.anthropic.com/v1/messages".into()),
            api_key_env: Some("ANTHROPIC_API_KEY".into()),
            max_tokens: 4096,
            temperature: 0.3,
        }
    }

    pub fn openai(model: impl Into<String>) -> Self {
        Self {
            kind: ProviderKind::OpenAI,
            model: Some(model.into()),
            endpoint: Some("https://api.openai.com/v1/chat/completions".into()),
            api_key_env: Some("OPENAI_API_KEY".into()),
            max_tokens: 4096,
            temperature: 0.3,
        }
    }
}

pub trait AgentProvider: Send + Sync {
    fn kind(&self) -> ProviderKind;
    fn config(&self) -> &ProviderConfig;
    fn complete(&self, prompt: &str) -> Result<String, String>;
    fn is_available(&self) -> bool;
}

pub struct NativeProvider {
    config: ProviderConfig,
}

impl NativeProvider {
    pub fn new() -> Self {
        Self { config: ProviderConfig::native() }
    }
}

impl Default for NativeProvider {
    fn default() -> Self { Self::new() }
}

impl AgentProvider for NativeProvider {
    fn kind(&self) -> ProviderKind { ProviderKind::Native }
    fn config(&self) -> &ProviderConfig { &self.config }
    fn complete(&self, prompt: &str) -> Result<String, String> {
        Ok(format!("[native-agent response to: {}]", &prompt[..prompt.len().min(80)]))
    }
    fn is_available(&self) -> bool { true }
}
