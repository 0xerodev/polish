use thiserror::Error;

#[derive(Debug, Error)]
pub enum StateError {
    #[error("invalid transition from '{from}' on event '{event}'")]
    InvalidTransition { from: String, event: String },
    #[error("transition guard failed in state '{state}'")]
    GuardFailed { state: String },
}
