#[macro_use]
pub extern crate failure;
#[macro_use]
extern crate human_panic;
#[macro_use]
extern crate clap;

pub mod utils;
pub mod commands;

use clap::App;
use clap::AppSettings;

use utils::error::Result;

pub fn start() -> Result<()> {
    // Setup human-panic
    setup_panic!();

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
