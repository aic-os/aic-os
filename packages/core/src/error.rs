use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("chain not supported: {0}")]
    ChainNotSupported(String),

    #[error("not implemented yet: {0}")]
    NotImplemented(&'static str),

    #[error("rpc error: {0}")]
    Rpc(String),

    #[error("invalid address: {0}")]
    InvalidAddress(String),

    #[error("settings error: {0}")]
    Settings(String),

    #[error("wallet error: {0}")]
    Wallet(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, CoreError>;
