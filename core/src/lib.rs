#[macro_use]
pub extern crate failure;

pub mod utils;

use utils::error::Result;

pub fn start() -> Result<()> {
    Ok(())
}
