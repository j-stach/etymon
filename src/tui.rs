
use headless_chrome as chrome;
use ratatui::{prelude::*, widgets::*, Terminal, backend::CrosstermBackend as Backend};

use std::collections::VecDeque;

use super::browser::*;

// TODO Expandable media types: Image, video?


/// Holistic frontend state & events.
pub struct Tui {
    pub terminal: Terminal<Backend<std::io::Stdout>>,
    pub display: TuiDisplay,
    pub cursor_cache: (u16, u16),
} impl Tui {

    /// Initializes the TUI on startup. Panics if a new terminal can't be generated.
    pub fn init() -> Tui {
        crossterm::terminal::enable_raw_mode().unwrap();
        let screen = std::io::stdout();
        if let Ok(terminal) = Terminal::new(Backend::new(screen)) {
            let display = TuiDisplay::new();
            Self { terminal, display, cursor_cache: (0, 0)}
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

    /// Stores the cursor position to allow redrawing of display.
    pub fn cache_cursor(&mut self) -> Result<(), anyhow::Error>{
        self.cursor_cache = self.terminal.get_cursor()?;
        Ok(())
    }

    /// Redraw the TUI screen according to its Display layout.
    pub fn draw(&mut self) -> Result<(), anyhow::Error> {
        self.terminal.draw(|frame| {
            self.display.render(frame).expect("Display renders to frame."); // TODO Handle draw error with logging?
            let (x, y) = self.cursor_cache;
            frame.set_cursor(x, y); // TODO DEBUG, causes reset to 0, 0 on redraw.
        })?;
        Ok(())
    }

    /// TBD Runs destructors and resets terminal changes.
    pub fn quit(&mut self) -> Result<(), anyhow::Error> {
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
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

        let tab_content = Block::default().borders(Borders::ALL).title("Page Content");
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
        self.dom = dom.collapse_phantoms();
        self.title = title.to_owned();
        self.update_layout();
        Ok(()) // TBD
    }

    pub fn update_layout(&mut self) {

        let nav = self.dom.isolate_nav_elements();
        // TBD Isolate nav elements: Find top nav bar & sidebar
        // then, in descending order, flatten the tree into a stack of paragraphs



        let layout = Layout::default();
        self.layout = layout;
    }


}




//-------------------------------------------------------------------------------------





/// Represents an HTML element tree as a TUI object.
#[derive(Clone)]
pub struct TuiNode {
    pub data: TuiNodeData,
    pub children: Vec<TuiNode>
} impl TuiNode {
    pub fn collapse_phantoms(mut self) -> Self {
        // TBD Does it matter if the root node is a Phantom?
        for (c, child) in self.children.iter().enumerate() {
            child.collapse_phantoms();
            if child.data.is_phantom() {
                let (front, back) = self.children.split_at(c);
                let (mut front, mut back) = (front.to_vec(), back.to_vec());
                let c = front.pop().expect("Removed phantom");
                let mut orphans = c.children.clone();
                front.append(&mut orphans);
                front.append(&mut back);
                self.children = front;
            }
        }
        self
    }

    pub fn isolate_nav_elements(&mut self) -> Vec<TuiNode> {
        let nav_elems = vec![];
        // TODO
        // for each in children, if it is a nav element
        // remove that child branch and push to nav_elems
        nav_elems
    }
}
impl std::default::Default for TuiNode {
    fn default() -> Self { TuiNode { data: TuiNodeData::Phantom, children: Vec::new() }}
}


#[derive(Clone)]
pub enum TuiNodeData {
    Element(TuiElement),
    Text(String),
    Comment(String),
    Phantom
} impl TuiNodeData {
    /// TODO Documentation
    pub fn is_phantom(&self) -> bool { match self { Self::Phantom => true, _ => false }}
}

#[derive(Clone)]
pub struct TuiElement {
    pub qual_name: html5ever::QualName,
    pub attributes: Vec<TuiAttribute>,
    pub contents: Vec<TuiNode>
} impl TuiElement {

}
impl PartialEq for TuiElement {
    fn eq(&self, other: &TuiElement) -> bool {
        self.qual_name == other.qual_name
    }
}


#[derive(Clone)]
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



// Other elements like text area

// popup dialog






















