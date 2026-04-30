use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("render error: {0}")]
    Render(String),
    #[error("invalid html structure: {0}")]
    Structure(String),
    #[error("forbidden raw html: use SafeHtml wrapper")]
    ForbiddenRaw,
}
