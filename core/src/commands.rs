use super::hazard;

use utils::app_config::AppConfig;
use utils::error::Result;

/// Show the configuration file
pub fn hazard() -> Result<()> {
    // Generate, randomly, True or False
    let random_hazard: bool = hazard::generate_hazard()?;


    if random_hazard {
    tui::display_dialog("You got it right!")?;
    } else {
    tui::display_dialog("You got it wrong!")?;
    }

    Ok(())
}

/// Show the configuration file
pub fn config() -> Result<()> {
    let config = AppConfig::fetch()?;
    println!("{:#?}", config);

    Ok(())
}
