use super::hazard;
use super::utils::config::AppConfig;
use super::utils::error::Result;

pub fn match_cmd(cli_matches: clap::ArgMatches) -> Result<()> {
    match cli_matches.subcommand() {
        ("hazard", _) => {
            cmd_hazard()?;
        }
        ("config", _) => {
            cmd_config()?;
        }
        _ => {
            // Arguments are required by default (in Clap)
            // This section should never execute and thus
            // should probably be logged in case it executed.
        }
    }

    Ok(())
}

/// Show the configuration file
pub fn cmd_hazard() -> Result<()> {
    // Generate, randomly, True or False
    let random_hazard: bool = hazard::generate_hazard()?;

    if random_hazard {
        println!("You got it right!");
    } else {
        println!("You got it wrong!");
    }

    Ok(())
}

/// Show the configuration file
pub fn cmd_config() -> Result<()> {
    let config = AppConfig::get()?;
    println!("{:#?}", config);
    Ok(())
}
