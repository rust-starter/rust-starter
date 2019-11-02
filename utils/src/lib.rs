extern crate failure;
extern crate config;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate lazy_static;

// slog dependencies
#[macro_use]
extern crate slog;
extern crate slog_syslog;
extern crate slog_term;
extern crate slog_scope;
extern crate slog_async;

pub mod error;
pub mod log;
pub mod app_config;
