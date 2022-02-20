use slog::o;
use slog::Drain;
use slog_journald::JournaldDrain;
use slog_syslog::Facility;

use super::error::Result;

pub fn setup_logging() -> Result<slog_scope::GlobalLoggerGuard> {
    // Setup Logging
    let guard = slog_scope::set_global_logger(default_root_logger()?);
    let _log_guard = slog_stdlog::init()?;

    Ok(guard)
}

pub fn default_root_logger() -> Result<slog::Logger> {
    // Create drains
    let drain = slog::Duplicate(default_discard()?, default_discard()?).fuse();

    // Merge drains
    #[cfg(feature = "termlog")]
    let drain = slog::Duplicate(default_term_drain().unwrap_or(default_discard()?), drain).fuse();
    #[cfg(feature = "syslog")]
    let drain = slog::Duplicate(default_syslog_drain().unwrap_or(default_discard()?), drain).fuse();
    #[cfg(feature = "journald")]
    let drain = slog::Duplicate(
        default_journald_drain().unwrap_or(default_discard()?),
        drain,
    )
    .fuse();

    // Create Logger
    let logger = slog::Logger::root(drain, o!("who" => "rust-starter"));

    // Return Logger
    Ok(logger)
}

fn default_discard() -> Result<slog_async::Async> {
    let drain = slog_async::Async::default(slog::Discard);

    Ok(drain)
}

// term drain: Log to Terminal
fn default_term_drain() -> Result<slog_async::Async> {
    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let term = slog_term::FullFormat::new(plain);

    let drain = slog_async::Async::default(term.build().fuse());

    Ok(drain)
}

// syslog drain: Log to syslog
fn default_syslog_drain() -> Result<slog_async::Async> {
    let syslog = slog_syslog::unix_3164(Facility::LOG_USER)?;

    let drain = slog_async::Async::default(syslog.fuse());

    Ok(drain)
}

fn default_journald_drain() -> Result<slog_async::Async> {
    let journald = JournaldDrain.ignore_res();
    let drain = slog_async::Async::default(journald);

    Ok(drain)
}
