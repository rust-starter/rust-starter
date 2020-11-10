#[cfg(not(debug_assertions))]
use human_panic::setup_panic;

#[cfg(debug_assertions)]
extern crate better_panic;

#[macro_use]
extern crate log;

use utils::app_config::AppConfig;
use utils::error::Result;

fn main() -> Result<()> {
    // Human Panic. Only enabled when *not* debugging.
    #[cfg(not(debug_assertions))]
    {
        setup_panic!();
    }

    // Better Panic. Only enabled *when* debugging.
    #[cfg(debug_assertions)]
    {
        better_panic::Settings::debug()
            .most_recent_first(false)
            .lineno_suffix(true)
            .verbosity(better_panic::Verbosity::Full)
            .install();
    }

    // Setup Logging
    //
    // TODO: This code should probably be included in utils::logger::setup_logging
    // The problem is that global variable is not set correct if this code is
    // executed from a sub-crate. I'm not sure if it is possible to "import"
    // this global variable to the root of the project and initialize it in
    // the utils crate.
    //
    //utils::logger::setup_logging()?;
    let _guard = slog_scope::set_global_logger(utils::logger::default_root_logger()?);
    let _log_guard = slog_stdlog::init()?;

    // Initialize Configuration
    let config_contents = include_str!("resources/default_config.toml");
    AppConfig::init(Some(config_contents))?;

    // Match Commands
    cli::cli_match()
}
