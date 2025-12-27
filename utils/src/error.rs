use thiserror::Error;

/// Result alias
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for this library.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Clap error: {0}")]
    Clap(#[from] clap::Error),

    #[error("Logger error: {0}")]
    Logger(#[from] log::SetLoggerError),

    #[error("Lock poisoned: {lock} - another thread panicked while holding this lock")]
    Poison { lock: String },

    #[error("{msg}")]
    Custom { msg: String },
}

impl Error {
    /// Create a new custom error.
    pub fn new(msg: &str) -> Self {
        Error::Custom {
            msg: msg.to_string(),
        }
    }

    /// Create a poison error for a specific lock.
    pub fn poison(lock: &str) -> Self {
        Error::Poison {
            lock: lock.to_string(),
        }
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(_err: std::sync::PoisonError<T>) -> Self {
        Error::Poison {
            lock: "unknown".to_string(),
        }
    }
}
