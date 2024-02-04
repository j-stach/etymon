
use headless_chrome as chrome;
use ratatui::{prelude::*, widgets::*, Terminal, backend::CrosstermBackend as Backend};

use std::collections::VecDeque;

use super::browser::*;

// TODO Expandable media types: Image, video?


/// Holistic frontend state & events.
pub struct Tui {
    pub terminal: Terminal<Backend<std::io::Stdout>>,
    pub display: TuiDisplay,
} impl Tui {

    /// Initializes the TUI on startup. Panics if a new terminal can't be generated.
    pub fn init() -> Tui {
        crossterm::terminal::enable_raw_mode().unwrap();
        let screen = std::io::stdout();
        if let Ok(terminal) = Terminal::new(Backend::new(screen)) {
            let display = TuiDisplay::new();
            Self { terminal, display }
        } else { panic!("Fatal Error: `Tui::init()` failed to create frontend!") }
    }

    /// Moves the cursor within bounds.
    pub fn move_cursor(&mut self, direction: CursorDirection) -> Result<(), anyhow::Error> {
        let cursor = self.terminal.get_cursor()?;
        use CursorDirection::*;
        match direction {
            Up    => if cursor.1 > 0 { self.terminal.set_cursor(cursor.0, cursor.1 - 1)? },
            Down  => self.terminal.set_cursor(cursor.0, cursor.1 + 1)?,
            Left  => if cursor.0 > 0 { self.terminal.set_cursor(cursor.0 - 1, cursor.1)? },
            Right => self.terminal.set_cursor(cursor.0 + 1, cursor.1)?,
        }
        Ok(())
    }

    /// Redraw the TUI screen according to its Display layout.
    pub fn draw(&mut self) -> Result<(), anyhow::Error> {
        self.terminal.draw(|frame| {
            self.display.render(frame).expect("Display renders to frame."); // TODO Handle draw error with logging?
            frame.set_cursor(0,0); // TODO DEBUG
        })?;
        Ok(())
    }

    // TODO
    pub async fn handle_input(&self) {

    }

    pub fn quit(&mut self) -> Result<(), anyhow::Error> {
        crossterm::terminal::disable_raw_mode().expect("Disable raw mode during quit");
        todo!{"Needs to clean up terminal events & state"}; // TODO
    }
}

/// Abstraction of cursor path options for future expansion.
/// TBD Jumping between widgets, for example.
#[derive(Clone, Copy)]
pub enum CursorDirection { Up, Down, Left, Right }


/// Represents visual aspects of the UI to be rendered.
pub struct TuiDisplay {
    pub navbar: TuiNavbar,
    pub tabs: VecDeque<TuiTab>,
    pub current_tab: usize,
} impl TuiDisplay {

    /// Struct to organize the state and properties of widgets to be rendered.
    pub fn new() -> TuiDisplay {
        TuiDisplay { navbar: TuiNavbar::new(), tabs: VecDeque::new(), current_tab: 0 }
    }

    /// Gets titles for all active tabs.
    pub fn tab_titles(&self) -> Vec<String> { self.tabs.iter().map(|t| t.title.clone()).collect() }

    /// Adds a new TuiTab to the register. TuiNavbar has to be redrawn separately for change to become visible.
    pub fn new_tab(&mut self, id: &str, title: &str) -> Result<(), anyhow::Error> {
        self.tabs.push_back(TuiTab::new(id, title)); // TODO Push tab to spec position
        Ok(())
    }

    /// Extracts HTML from chrome tab and rebuilds TuiTab to reflect changes.
    pub fn update_tab(&mut self, chrome_tab: &chrome::Tab) -> Result<(), anyhow::Error> {
        let tab_id = chrome_tab.get_target_id();
        let html = chrome_tab.get_content()?.to_string();
        filter_dom_html(&html);

        let tab = self.tabs.iter_mut().find(|t| &t.id == tab_id).expect("Finds tab in record");
        tab.update(parse_content(&html).to_tui(), &chrome_tab.get_title()?)?;

        Ok(())
    }

    /// Renders the Display to the current screen.
    pub fn render(&mut self, frame: &mut Frame) -> Result<(), anyhow::Error> {
        let bar_h = match self.navbar.show_tabs { true => 2, false => 1 };
        let layout = {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(100), // Page view section
                    Constraint::Min(bar_h),      // Navbar height
                ])
                .split(frame.size())
        };

        let navbar_constraints = match self.navbar.show_tabs {
            true => [ Constraint::Percentage(50), Constraint::Percentage(50) ],
            false => [ Constraint::Percentage(100), Constraint::Percentage(0) ]
        };

        let navbar_layout = {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints(navbar_constraints)
                .split(layout[1])
        };

        let tab_content = Block::default().borders(Borders::ALL).title("Page COntent");
        frame.render_widget(tab_content, layout[0]);

        let tab_titles = self.tab_titles();
        let tab_list = self.navbar.render_tab_list(tab_titles);
        let command_line = self.navbar.render_command_line();

        if self.navbar.show_tabs {
            frame.render_widget(tab_list, navbar_layout[0]);
            frame.render_widget(command_line, navbar_layout[1]);
        } else {
            frame.render_widget(command_line, navbar_layout[0]);
        }

        Ok(())
    }
}






pub struct TuiNavbar {
    pub show_tabs: bool,
    //pub show_bookmarks: bool,
} impl TuiNavbar {

    pub fn new() -> Self { TuiNavbar { show_tabs: true }}

    pub fn render_tab_list(&self, tab_titles: Vec<String>) -> Tabs {
       Tabs::new(tab_titles)
            .block(Block::default().borders(Borders::NONE))
            .style(Style::default().white().bg(Color::Blue))
            .highlight_style(Style::default().yellow())
            .select(0)
            .divider(symbols::DOT)
            .padding("┌", "┐") // TBD No padding for active tab, has top 3 sides border instead
    }

    pub fn render_command_line(&self) -> Paragraph {
        Paragraph::new("Command line interface").style(Style::default().white().bg(Color::Black))
    }
}

/// Represents the associated chrome tab and webpage as a TUI object.
pub struct TuiTab {
    pub id: String,
    pub title: String,
    pub dom: TuiNode,
    pub layout: Layout
} impl TuiTab {

    /// Creates a new tab
    pub fn new(id: &str, title: &str) -> TuiTab {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            dom: TuiNode::default(),
            layout: Layout::default()
        }
    }

    pub fn update(&mut self, dom: TuiNode, title: &str) -> Result<(), anyhow::Error> {
        self.dom = dom;
        self.title = title.to_owned();
        self.update_layout();
        Ok(()) // TBD
    }

    pub fn update_layout(&mut self) {
        let layout = Layout::default();
        // TBD use dom to create layout?
        // Find Phantom nodes & collapse
        // Then when rendering, skip phantom nodes when calculating index
        self.layout = layout;
    }

}










/// Represents an HTML element tree as a TUI object.
#[derive(Default)]
pub struct TuiNode {
    pub children: Vec<TuiNode>
}

pub enum TuiNodeData {
    Element(TuiElement),
    Text(String),
    Comment(String),
    Phantom, // TBD All but Phantom are rendered with a border/block
}

pub struct TuiElement {
    pub qual_name: html5ever::QualName,
    pub attributes: Vec<TuiAttribute>,
    pub contents: Vec<TuiNode>
} impl TuiElement {}

pub struct TuiAttribute(pub String);

/*
 Display layout-
    Tab layout-
        Sidebar-
        Page view-
    Bottom bar-
        Status bar-
        Tab list-
        Command line/search bar-
*/


// TBD Isolate nav elements
// Find top nav bar & sidebar
























