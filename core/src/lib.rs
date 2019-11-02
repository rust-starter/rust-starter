extern crate clap;

// This library is not required, and is used to generate
// random numbers for one of the example commands
extern crate rand;

pub mod commands;
pub mod hazard;

use cli;
use utils;

use utils::app_config::AppConfig;
use utils::error::Result;
use utils::log;

pub fn start() -> Result<()> { 
    // Get matches
    let cli_matches = cli::cli_config()?;

    // Merge clap config file if the value is set
    AppConfig::init(cli_matches.value_of("config"))?;

    // Matches Commands or display help
    commands::match_cmd(cli_matches)?;

    Ok(())
}
