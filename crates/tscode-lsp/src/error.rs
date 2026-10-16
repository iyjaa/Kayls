use thiserror::Error;

#[derive(Debug, Error)]
pub enum LspError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("LSP protocol error: {0}")]
    Protocol(String),

    #[error("Server not running")]
    ServerNotRunning,

    #[error("Server startup timeout")]
    StartupTimeout,

    #[error("Channel closed")]
    ChannelClosed,

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, LspError>;
