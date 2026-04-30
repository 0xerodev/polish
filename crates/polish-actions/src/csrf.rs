use rand::Rng;
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::Utc;

/// A signed CSRF token (session-bound, time-limited).
#[derive(Clone, Debug)]
pub struct CsrfToken {
    pub value: String,
    pub expires_at: i64,
}

impl CsrfToken {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let raw: [u8; 32] = rng.gen();
        let value = hex::encode(raw);
        Self { value, expires_at: Utc::now().timestamp() + 3600 }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.expires_at
    }
}

impl Default for CsrfToken {
    fn default() -> Self { Self::new() }
}

/// In-memory CSRF token store (single-use, time-limited).
#[derive(Clone, Default)]
pub struct CsrfStore {
    tokens: Arc<Mutex<HashMap<String, CsrfToken>>>,
}

impl CsrfStore {
    pub fn new() -> Self { Self::default() }

    /// Issue a new token for a session.
    pub fn issue(&self, session_id: &str) -> CsrfToken {
        let token = CsrfToken::new();
        let combined = format!("{}:{}", session_id, &token.value);
        let mut hasher = Sha256::new();
        hasher.update(combined.as_bytes());
        let signed_hex = hex::encode(hasher.finalize());
        let signed = CsrfToken {
            value: signed_hex.clone(),
            expires_at: token.expires_at,
        };
        let mut store = self.tokens.lock().unwrap();
        store.insert(signed_hex, signed.clone());
        // Prune expired
        store.retain(|_, v| !v.is_expired());
        signed
    }

    /// Validate and consume a token (single-use).
    pub fn validate(&self, token_value: &str) -> bool {
        let mut store = self.tokens.lock().unwrap();
        if let Some(t) = store.remove(token_value) {
            !t.is_expired()
        } else {
            false
        }
    }

    /// Check without consuming (for GET forms).
    pub fn check(&self, token_value: &str) -> bool {
        let store = self.tokens.lock().unwrap();
        store.get(token_value).map(|t| !t.is_expired()).unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csrf_issue_and_validate() {
        let store = CsrfStore::new();
        let token = store.issue("session-1");
        assert!(store.validate(&token.value));
        // Single-use: second validate should fail
        assert!(!store.validate(&token.value));
    }
}
