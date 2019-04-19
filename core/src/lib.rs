#[macro_use]
pub extern crate failure;
#[macro_use]
extern crate human_panic;

pub mod utils;

use utils::error::Result;

pub fn start() -> Result<()> {
    // Setup human-panic
    setup_panic!();

    Ok(())
}
