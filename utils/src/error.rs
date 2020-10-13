use std::result;
use thiserror::Error;


/// A type alias that forces the usage of the custom error type.
pub type Result<T> = result::Result<T, GlobalError>;

/// Custom error type for handling errors.
#[derive(Error, Debug)]
pub enum GlobalError {
    #[error("Configuration Error")]
    ConfigError(#[from] config::ConfigError),
    #[error("Poison Error")]
    PoisonError,
    #[error("IO Error")]
    IoError(#[from] std::io::Error),
    #[error("Clap Error")]
    ClapError(#[from] clap::Error),
    #[error("Undefined Error")]
    Undefined,
}

impl<T> From<std::sync::PoisonError<T>> for GlobalError {
    fn from(_err: std::sync::PoisonError<T>) -> Self {
        GlobalError::PoisonError
    }
}
