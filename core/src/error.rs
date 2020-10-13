use utils::error::Result;
use std::fs::File;

/// Return, randomly, true or false
pub fn simulate_error() -> Result<()> {
    // Trigger an error
    File::open("thisfiledoesnotexist")?;

    Ok(())
}
