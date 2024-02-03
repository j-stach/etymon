
use headless_chrome as chrome;
use super::display as tui;

use termion::input::TermRead;

pub use crate::ui;

/// Handles the communication between headless Chrome browser and terminal UI.
pub struct Etymon {
    /// Chrome backend to handle web requests.
    pub browser: chrome::Browser,
    /// TUI frontend handle to display webpage HTML.
    pub display: tui::Display,
    pub mode: ui::UIMode,
} impl Etymon {

    /// Open a new instance of Etymon from configuration parameters.
    pub fn init() -> Result<Etymon, anyhow::Error> {
        let browser = chrome::Browser::default()?;  // TBD Config
        let display = tui::Display::new()?;      // TBD Config
        Ok(Self { browser, display, mode: ui::UIMode::View })
    }

    /// Get access to the Chrome browser backend.
    pub fn chrome(&self) -> &chrome::Browser { &self.browser }

    /// Get mutable access to the TUI display handle.
    pub fn tui(&mut self) -> &mut tui::Display { &mut self.display }

    /// Creates a new tab in the browser backend.
    pub fn new_tab(&mut self, url: Option<&str>) -> Result<(), anyhow::Error> {
        let tab = self.chrome().new_tab()?;
        let page: String;
        if let Some(url) = url { page = url.to_owned() }
        else { let url = crate::CONFIG.homepage.clone(); page = url }

        self.tui().new_tab(tab.get_target_id())?;
        self.load_page(&tab, &page)?;
        Ok(())
    }

    /// Loads URL to browser tab and updates representation in TUI.
    pub fn load_page(&mut self, tab: &chrome::Tab, url: &str) -> Result<(), anyhow::Error> {
        tab.navigate_to(url)?
            .wait_until_navigated()?;
        self.tui().update_tab(&tab)?; // TODO Test
        Ok(())
    }

    #[allow(non_snake_case)]
    pub fn HANDLE_USER_INPUT(&mut self) -> Result<(), anyhow::Error> {
        let keys = std::io::stdin().keys();
        self.process_keys(keys)?;
        Ok(())
    }

}
