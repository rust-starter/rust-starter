#[cfg(not(debug_assertions))]
#[macro_use]
extern crate human_panic;

use utils::error::Result;
use utils::log;

fn main() -> Result<()> {
    // Human Panic. Only enabled when *not* debugging.
    #[cfg(not(debug_assertions))]
    {
        setup_panic!();
    }

    // Setup Logging
    log::setup_logging()?;

    cli::cli_match()
}
