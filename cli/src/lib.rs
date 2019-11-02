#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};

use utils::error::Result;

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
