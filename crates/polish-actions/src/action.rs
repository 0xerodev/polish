use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;

/// What the server returns after executing an action.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ActionOutput {
    /// Replace the page with a new render.
    Replace { page_html: String },
    /// HTTP redirect.
    Redirect { url: String },
    /// Render error page / inline error.
    Error { message: String, code: u16 },
    /// Partial fragment update.
    Fragment { target_id: String, html: String },
    /// Hand off to a streaming endpoint.
    Stream { url: String },
    /// File download.
    Download { filename: String, content_type: String, data: Vec<u8> },
}

/// Result of executing a server action.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionResult {
    pub action_id: String,
    pub outcome: ActionOutput,
    pub timestamp: i64,
    pub audit_event: Option<AuditEvent>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditEvent {
    pub action: String,
    pub user_id: Option<String>,
    pub fields: HashMap<String, String>,
    pub timestamp: i64,
    pub outcome: String,
}

/// Context passed to every action handler.
#[derive(Clone, Debug, Default)]
pub struct ActionContext {
    pub session_id: String,
    pub user_id: Option<String>,
    pub permissions: Vec<String>,
    pub client_ip: String,
    pub audit: bool,
    pub idempotency_key: Option<String>,
}

impl ActionContext {
    pub fn new() -> Self { Self::default() }

    pub fn with_session(mut self, id: impl Into<String>) -> Self {
        self.session_id = id.into(); self
    }

    pub fn with_user(mut self, uid: impl Into<String>, perms: Vec<String>) -> Self {
        self.user_id = Some(uid.into());
        self.permissions = perms;
        self
    }

    pub fn with_ip(mut self, ip: impl Into<String>) -> Self {
        self.client_ip = ip.into(); self
    }

    pub fn has_permission(&self, perm: &str) -> bool {
        self.permissions.iter().any(|p| p == perm || p == "admin")
    }

    pub fn require_auth(&self) -> Result<&str, ActionError> {
        self.user_id.as_deref().ok_or(ActionError::Unauthorized)
    }
}

use crate::error::ActionError;

/// Builder for ActionResult.
pub struct Action;

impl Action {
    pub fn replace(html: impl Into<String>) -> ActionResult {
        ActionResult {
            action_id: Uuid::new_v4().to_string(),
            outcome: ActionOutput::Replace { page_html: html.into() },
            timestamp: Utc::now().timestamp(),
            audit_event: None,
        }
    }

    pub fn redirect(url: impl Into<String>) -> ActionResult {
        ActionResult {
            action_id: Uuid::new_v4().to_string(),
            outcome: ActionOutput::Redirect { url: url.into() },
            timestamp: Utc::now().timestamp(),
            audit_event: None,
        }
    }

    pub fn error(message: impl Into<String>, code: u16) -> ActionResult {
        ActionResult {
            action_id: Uuid::new_v4().to_string(),
            outcome: ActionOutput::Error { message: message.into(), code },
            timestamp: Utc::now().timestamp(),
            audit_event: None,
        }
    }

    pub fn fragment(target_id: impl Into<String>, html: impl Into<String>) -> ActionResult {
        ActionResult {
            action_id: Uuid::new_v4().to_string(),
            outcome: ActionOutput::Fragment { target_id: target_id.into(), html: html.into() },
            timestamp: Utc::now().timestamp(),
            audit_event: None,
        }
    }
}

impl ActionResult {
    pub fn with_audit(mut self, event: AuditEvent) -> Self {
        self.audit_event = Some(event);
        self
    }
}
