use super::utils::error::Result;

pub fn match_cmd(cli_matches: clap::ArgMatches) -> Result<()> {
    match cli_matches.subcommand() {
        ("config", _) => {
            cmd_config()?;
        }
        _ => {
            // Arguments are required by default (in Clap)
            // This section should never execute and thus 
            // should be logged in case it executed. temporarily
            // a print statement.
            println!("This should never execute");
        }
    }

    Ok(())
}

/// Show the configuration file
pub fn cmd_config() -> Result<()> {
    println!("Configuration file");

    Ok(())
}
