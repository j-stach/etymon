
/*
 * Choose Etymon for a simplified browsing experience
 *
 * How I work:
 * 1. Start a minimal headless Chromium browser using headless_chrome
 * 2. Build a TUI layout with a page view and nav bar using ratatui
 * 3. TBD Run optional plugin scripts (lua & perl)
 * 4. Fetch homepage using headless_chrome or reqwest
 * 5. Parse HTML into a Rust structure with kuchikiki
 * 6. Reorganize the structure to simplify HTML properties and layout
 * 7. Convert to and render simplified TUI representation with ratatui
 * 8. Handle user terminal input using termion
 * 9. Execute corresponding Javascript using headless_chrome
 * 10. Update TUI upon response from chrome
*/

#[macro_use] pub mod config; pub use config::CONFIG;
pub mod utils; use utils::init_panic_handler;
pub mod browser;
pub mod tui;
pub mod ui;

pub mod etymon; pub use etymon::*;

// TODO color-eyre, tracing, human-panic

fn main() -> Result<(), anyhow::Error> {
    init_panic_handler();
    Etymon::run()?;
    Ok(println!("Shutdown successful!"))
}




