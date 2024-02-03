
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

pub mod config; pub use config::CONFIG;
pub mod browser;
pub mod display;
pub mod ui;

pub mod etymon; pub use etymon::*;

lazy_static::lazy_static!{
    pub static ref RUN: std::sync::Mutex<bool> = std::sync::Mutex::new(true);
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    println!("{}", termion::clear::All);

    let mut etymon = Etymon::init()?;
    etymon.new_tab(None)?;
    while *RUN.lock().expect("Should get the lock for the run switch.") == true {
        etymon.display.render()?;
        etymon.HANDLE_USER_INPUT()?;
        // handle user input from tui
    }

    // loop {}

    // accept user input
    // if send via headless, refresh tab


    Ok(println!("Hello, world!"))
}







