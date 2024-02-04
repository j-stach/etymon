
use headless_chrome as chrome;
use super::tui;

use crossterm::{execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};

pub use crate::ui;

/// Handles the communication between headless Chrome browser and terminal UI.
pub struct Etymon {
    /// Chrome backend to handle web requests.
    pub browser: chrome::Browser,
    /// TUI frontend handle to tui webpage HTML.
    pub tui: tui::Tui,
    /// The app will gracefully quit when this evaluates to true.
    pub should_quit: bool,
    /// Holds the current UI mode: View or Write.
    pub mode: ui::UIMode,
} impl Etymon {

    /// Create a new instance of Etymon from configuration parameters.
    pub fn init() -> Result<Etymon, anyhow::Error> {
        let browser = chrome::Browser::default()?;
        let tui = tui::Tui::init();
        let mut etymon = Self { browser, tui, should_quit: false, mode: ui::UIMode::View };
        etymon.new_tab(None)?;
        Ok(etymon)
    }

    /// Build, start, and run the application.
    pub fn run() -> Result<(), anyhow::Error> {
        execute!(std::io::stdout(), EnterAlternateScreen)?;

        let mut etymon = Etymon::init()?;
        loop {
            etymon.tui.draw()?;
            etymon.handle_ui()?;
            if etymon.should_quit { break }
        }

        if crossterm::terminal::is_raw_mode_enabled()? { crossterm::terminal::disable_raw_mode()? }
        execute!(std::io::stdout(), LeaveAlternateScreen)?;
        Ok(())
    }

    /// Get access to the Chrome browser backend.
    pub fn chrome(&self) -> &chrome::Browser { &self.browser }
    /// Get mutable access to the TUI tui handle.
    pub fn tui(&mut self) -> &mut tui::Tui { &mut self.tui }

    /// Creates a new tab in the browser backend.
    pub fn new_tab(&mut self, url: Option<&str>) -> Result<(), anyhow::Error> {
        let tab = self.chrome().new_tab()?;

        let page: String;
        if let Some(url) = url { page = url.to_owned() }
        else { let url = crate::CONFIG.homepage.clone(); page = url }

        self.tui().display.new_tab(tab.get_target_id(), &tab.get_title()?)?;
        self.load_page(&tab, &page)?;
        Ok(())
    }

    /// Loads URL to browser tab and updates representation in TUI.
    pub fn load_page(&mut self, tab: &chrome::Tab, url: &str) -> Result<(), anyhow::Error> {
        tab.navigate_to(url)?
            .wait_until_navigated()?;
        self.tui().display.update_tab(&tab)?; // TODO Test
        Ok(())
    }

    /// Gracefully exit Etymon to terminal.
    pub fn quit(&mut self) {
        self.should_quit = true;
        if let Err(_) = self.tui().quit() {
            panic!("Error: This panic was used to exit the terminal. Toodaloo!")
        }
    }

}
