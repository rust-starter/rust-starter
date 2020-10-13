use clap::{App, AppSettings, Arg};
use clap::{crate_version, crate_description, crate_authors};

use core::commands;
use utils::app_config::AppConfig;
use utils::error::Result;

/// Match commands
pub fn cli_match() -> Result<()> {
    // Get matches
    let cli_matches = cli_config()?;

    // Merge clap config file if the value is set
    AppConfig::merge_config(cli_matches.value_of("config"))?;

    // Matches Commands or display help
    match cli_matches.subcommand_name() {
        Some("hazard") => {
            commands::hazard()?;
        }
        Some("error") => {
            commands::simulate_error()?;
        }
        Some("config") => {
            commands::config()?;
        }
        _ => {
            // Arguments are required by default (in Clap)
            // This section should never execute and thus
            // should probably be logged in case it executed.
        }
    }
    Ok(())
}

/// Configure Clap
/// This function will configure clap and match arguments
pub fn cli_config() -> Result<clap::ArgMatches> {
    let cli_app = App::new("rust-starter")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!("\n"))
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .about("Set a custom config file")
                .takes_value(true),
        )
        .subcommand(App::new("hazard").about("Generate a hazardous occurance"))

        .subcommand(App::new("error").about("Simulate an error"))
        .subcommand(App::new("config").about("Show Configuration"));

    // Get matches
    let cli_matches = cli_app.get_matches();

    Ok(cli_matches)
}
