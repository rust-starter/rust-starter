#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};

use core::commands;
use utils::app_config::AppConfig;
use utils::error::Result;

/// Match commands
pub fn cli_match() -> Result<()> {
    // Get matches
    let cli_matches = cli_config()?;

    // Merge clap config file if the value is set
    AppConfig::init(cli_matches.value_of("config"))?;

    // Matches Commands or display help
    match cli_matches.subcommand() {
        ("hazard", _) => {
            commands::hazard()?;
        }
        ("config", _) => {
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
pub fn cli_config<'a>() -> Result<clap::ArgMatches<'a>> {
    let cli_app = App::new("rust-starter")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!("\n"))
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Set a custom config file")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name("hazard").about("Generate a hazardous occurance"))
        .subcommand(SubCommand::with_name("config").about("Show Configuration"));

    // Get matches
    let cli_matches = cli_app.get_matches();

    Ok(cli_matches)
}
