use std::result;
use std::backtrace::Backtrace;
use thiserror::Error;


/// A type alias that forces the usage of the custom error type.
pub type Result<T> = result::Result<T, GlobalError>;

/// Custom error type for handling errors.
#[derive(Error, Debug)]
pub enum GlobalError {
    #[error("Configuration Error")]
    ConfigError {
        #[from]
        source: config::ConfigError,
        backtrace: Backtrace,
    },
    #[error("Poison Error")]
    PoisonError,
    #[error("IO Error")]
    IoError {
        #[from]
        source: std::io::Error,
        backtrace: Backtrace,
    },
    #[error("Clap Error")]
    ClapError {
        source: clap::Error,
        backtrace: Backtrace,
    },
    #[error("Undefined Error")]
    Undefined {
        backtrace: Backtrace,
    },
}

impl<T> From<std::sync::PoisonError<T>> for GlobalError {
    IoError,
    ClapError,
    LoggerError,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Undefined Error")
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: failure::Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

impl From<config::ConfigError> for Error {
    fn from(err: config::ConfigError) -> Self {
        Error {
            inner: err.context(ErrorKind::ConfigError),
        }
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(_err: std::sync::PoisonError<T>) -> Self {
        GlobalError::PoisonError
    }
}

impl From<log::SetLoggerError> for Error {
    fn from(err: log::SetLoggerError) -> Self {
        Error {
            inner: err.context(ErrorKind::LoggerError),
        }
    }
}
