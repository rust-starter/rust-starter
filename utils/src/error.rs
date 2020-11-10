use std::fmt;
use std::result;

use failure::{Backtrace, Context, Fail};

/// A type alias that forces the usage of the custom error type.
pub type Result<T> = result::Result<T, Error>;

/// Custom error type for handling errors.
#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        *self.inner.get_context()
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }
    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Fail)]
pub enum ErrorKind {
    ConfigError,
    PoisonError,
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
        Error {
            inner: Context::from(ErrorKind::PoisonError),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error {
            inner: err.context(ErrorKind::IoError),
        }
    }
}

impl From<clap::Error> for Error {
    fn from(err: clap::Error) -> Self {
        Error {
            inner: err.context(ErrorKind::ClapError),
        }
    }
}

impl From<log::SetLoggerError> for Error {
    fn from(err: log::SetLoggerError) -> Self {
        Error {
            inner: err.context(ErrorKind::LoggerError),
        }
    }
}
