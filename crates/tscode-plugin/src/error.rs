use thiserror::Error;

#[derive(Debug, Error)]
pub enum PluginError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("WASM error: {0}")]
    Wasm(String),

    #[error("Plugin not found: {0}")]
    NotFound(String),

    #[error("Plugin already loaded: {0}")]
    AlreadyLoaded(String),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, PluginError>;
