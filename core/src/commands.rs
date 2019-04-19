use super::utils::error::Result;

pub fn match_cmd(cli_matches: clap::ArgMatches) -> Result<()> {
    match cli_matches.subcommand() {
        ("config", _) => {
            cmd_config()?;
        }
        _ => {
            // Args Required
        }
    }

    Ok(())
}

/// Show the configuration file
pub fn cmd_config() -> Result<()> {
    println!("Configuration file");

    Ok(())
}
