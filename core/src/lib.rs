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

pub extern crate failure;

extern crate config;
extern crate rand;
extern crate slog_syslog;

pub mod commands;
pub mod hazard;
pub mod utils;

use clap::App;
use clap::AppSettings;
use slog::Drain;
use slog_syslog::Facility;
use std::sync::RwLock;

use utils::config::AppConfig;
use utils::error::Result;

lazy_static! {
    static ref SETTINGS: RwLock<AppConfig> = RwLock::new(AppConfig::new(None).unwrap());
}

pub fn start() -> Result<()> {
    #[cfg(not(debug_assertions))]
    {
        // Setup human-panic
        setup_panic!();
    }

    // Setup Logging
    let syslog = slog_syslog::unix_3164(Facility::LOG_USER)?;
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

    // Setup default Settings
    {
        let mut w = SETTINGS.write().unwrap();
        *w = AppConfig::new(cli_matches.value_of("config"))?;
    }

    // Matches Commands or display help
    commands::match_cmd(cli_matches)?;

    Ok(())
}
