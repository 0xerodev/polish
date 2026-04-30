use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ActionError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Rate limited — try again shortly")]
    RateLimited,
    #[error("Service unavailable")]
    ServiceUnavailable,
    #[error("Action failed")]
    Internal,
    #[error("Invalid CSRF token")]
    InvalidCsrf,
    #[error("Duplicate submission")]
    DuplicateSubmission,
}

impl ActionError {
    pub fn status_code(&self) -> u16 {
        match self {
            ActionError::Unauthorized => 401,
            ActionError::Forbidden => 403,
            ActionError::InvalidInput(_) => 422,
            ActionError::Conflict(_) => 409,
            ActionError::RateLimited => 429,
            ActionError::ServiceUnavailable => 503,
            ActionError::Internal => 500,
            ActionError::InvalidCsrf => 403,
            ActionError::DuplicateSubmission => 409,
        }
    }

    /// User-facing message. Never exposes internal details.
    pub fn user_message(&self) -> &str {
        match self {
            ActionError::Unauthorized => "Not authorized",
            ActionError::Forbidden => "Access denied",
            ActionError::InvalidInput(m) => m,
            ActionError::Conflict(m) => m,
            ActionError::RateLimited => "Too many requests — try again shortly",
            ActionError::ServiceUnavailable => "Service temporarily unavailable",
            ActionError::Internal => "Action failed",
            ActionError::InvalidCsrf => "Invalid request — please reload",
            ActionError::DuplicateSubmission => "Request already submitted",
        }
    }
}
