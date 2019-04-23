#[macro_use]
extern crate human_panic;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate slog;

pub extern crate failure;
extern crate rand;
extern crate slog_syslog;

pub mod commands;
pub mod hazard;
pub mod utils;

use clap::App;
use clap::AppSettings;
use slog::Drain;
use slog_syslog::Facility;

use utils::error::Result;

pub fn start() -> Result<()> {
    #[cfg(not(debug_assertions))]
    {
        // Setup human-panic
        setup_panic!();
    }

    // Setup Logging
    let syslog = slog_syslog::unix_3164(Facility::LOG_USER).unwrap();
    let logger = slog::Logger::root(syslog.fuse(), o!("who" => "rust-starter"));

    info!(logger, "testing logging");

    // Load Yaml configuration for Clap
    let yml = load_yaml!("resources/cli_config.yml");

    // Parses the Yaml configuration file
    let cli_app = App::from_yaml(yml)
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!("\n"));

    // Get matches
    let cli_matches = cli_app.get_matches();

    // Matches Commands or display help
    commands::match_cmd(cli_matches)?;

    Ok(())
}
