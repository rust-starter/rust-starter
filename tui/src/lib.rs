use cursive::Cursive;
use cursive::views::{Dialog, TextView};

use utils::error::Result;

pub fn display_dialog(hazard: &str) -> Result<()> {
    // Creates the cursive root - required for every application.
    let mut siv = Cursive::default();

    // Creates a dialog with a single "Quit" button
    siv.add_layer(
        Dialog::around(TextView::new(hazard))
            .title("Hazard!")
            .button("Quit", |s| s.quit()),
    );

    // Starts the event loop.
    siv.run();

    Ok(())
}
