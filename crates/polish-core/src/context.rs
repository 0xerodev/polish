use std::collections::HashMap;

/// Threading state through the render tree without global state.
#[derive(Clone, Default)]
pub struct RenderContext {
    pub nonce: Option<String>,
    pub base_url: String,
    pub lang: String,
    pub debug: bool,
    pub user_id: Option<String>,
    pub permissions: Vec<String>,
    data: HashMap<String, String>,
}

impl RenderContext {
    pub fn new() -> Self { Self::default() }

    pub fn with_nonce(mut self, nonce: impl Into<String>) -> Self {
        self.nonce = Some(nonce.into());
        self
    }

    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    pub fn with_lang(mut self, lang: impl Into<String>) -> Self {
        self.lang = lang.into();
        self
    }

    pub fn with_user(mut self, user_id: impl Into<String>, perms: Vec<String>) -> Self {
        self.user_id = Some(user_id.into());
        self.permissions = perms;
        self
    }

    pub fn set(&mut self, key: &str, value: impl Into<String>) {
        self.data.insert(key.to_string(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|s| s.as_str())
    }

    pub fn has_permission(&self, perm: &str) -> bool {
        self.permissions.iter().any(|p| p == perm || p == "admin")
    }

    pub fn is_authenticated(&self) -> bool {
        self.user_id.is_some()
    }

    pub fn lang_or_default(&self) -> &str {
        if self.lang.is_empty() { "en" } else { &self.lang }
    }
}
