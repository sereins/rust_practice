use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransportError {
    #[error("request error {0}")]
    ReqError(#[from] reqwest::Error),
    #[error("serde error {0}")]
    SerdeError(#[from] serde_json::Error),
}
