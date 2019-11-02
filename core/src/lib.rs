// This library is not required, and is used to generate
// random numbers for one of the example commands
extern crate rand;

pub mod commands;
pub mod hazard;

use utils::error::Result;

pub fn start() -> Result<()> {
    // does nothing

    Ok(())
}
