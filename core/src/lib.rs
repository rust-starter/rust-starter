#[cfg(not(debug_assertions))]
#[macro_use]
extern crate human_panic;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate slog;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate lazy_static;
extern crate config;
pub extern crate failure;
extern crate slog_syslog;
extern crate slog_term;
extern crate slog_scope;
extern crate slog_async;

// This library is not required, and is used to generate
// random numbers for one of the example commands
extern crate rand;

pub mod commands;
pub mod hazard;
pub mod utils;

use clap::App;
use clap::AppSettings;

use utils::config::AppConfig;
use utils::error::Result;
use utils::log;

pub fn start() -> Result<()> {
    // Human Panic. Only enabled when *not* debugging.
    #[cfg(not(debug_assertions))]
    {
        setup_panic!();
    }

    // Setup Logging
    let _guard = slog_scope::set_global_logger(log::default_root_logger()?);

    // Load Yaml configuration for Clap
    let yml = load_yaml!("resources/cli_config.yml");

    // Parses the Yaml configuration file
    let cli_app = App::from_yaml(yml)
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!("\n"));

    // Get matches
    let cli_matches = cli_app.get_matches_safe()?;

    // Merge clap config file if the value is set
    AppConfig::init(cli_matches.value_of("config"))?;

    // Matches Commands or display help
    commands::match_cmd(cli_matches)?;

    Ok(())
}
