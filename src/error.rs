use thiserror::Error;

#[derive(Debug, Error)]
pub enum LauncherError {
    #[error("JSON serialization/deserialization error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("FromUtf8Error Error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("Invalid response from Launcher")]
    InvalidResponse,
    #[error("Launcher Api Error: {0}")]
    ApiError(String),
}
