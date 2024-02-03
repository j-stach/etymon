
use ratatui::{prelude::*, widgets::*, backend::TermionBackend, Terminal};
use termion::screen::{AlternateScreen, IntoAlternateScreen};
use termion::raw::{RawTerminal, IntoRawMode};
use headless_chrome as chrome;

use std::collections::VecDeque;

use super::browser::*;

// TODO Expandable media types: Image, video?

pub struct Display {
    pub terminal: Terminal<TermionBackend<AlternateScreen<RawTerminal<std::io::Stdout>>>>,
    pub tabs: VecDeque<TuiTab>,
    pub navbar: TuiNavbar,
    pub needs_update: bool,
} impl Display {

    pub fn new() -> Result<Display, anyhow::Error> {
        let screen = std::io::stdout().into_raw_mode()?.into_alternate_screen()?;
        if let Ok(terminal) = Terminal::new(TermionBackend::new(screen)) {
            let display = Display {
                terminal,
                tabs: VecDeque::new(),
                navbar: TuiNavbar::new(),
                needs_update: false,
            };
            Ok(display)
        } else { Err(anyhow::anyhow!("Fatal Error: Failed to create display")) }
    }

    pub fn new_tab(&mut self, id: &str) -> Result<(), anyhow::Error> {
        self.tabs.push_back(TuiTab::new(id)); // TODO Push tab to spec position
        // TODO Add to navbar
        Ok(())
    }

    pub fn update_tab(&mut self, chrome_tab: &chrome::Tab) -> Result<(), anyhow::Error> {
        // TBD Get tab by ID, then create tui like so:
        let tab_id = chrome_tab.get_target_id();
        let html = chrome_tab.get_content()?.to_string();
        filter_dom_html(&html);

        let tab = self.tabs.iter_mut().find(|t| &t.id == tab_id).expect("Finds tab in record");
        tab.update(parse_content(&html).to_tui())?;

        Ok(())
    }

    pub fn tab_names(&self) -> Vec<String> {
        self.tabs.iter().map(|t| t.name.clone()).collect()
    }

    pub fn move_cursor(&mut self, direction: CursorDirection) -> Result<(), anyhow::Error> {
        let cursor = self.terminal.get_cursor()?;
        use CursorDirection::*;     // TODO Needs to be more fluid and enable diagonals/holding down
        match direction {
            Up    => if cursor.1 > 0 { self.terminal.set_cursor(cursor.0, cursor.1 - 1)? },
            Down  => self.terminal.set_cursor(cursor.0, cursor.1 + 1)?,
            Left  => if cursor.0 > 0 { self.terminal.set_cursor(cursor.0 - 1, cursor.1)? },
            Right => self.terminal.set_cursor(cursor.0 + 1, cursor.1)?,
        }
        Ok(())
    }

    pub fn render(&mut self) -> Result<(), anyhow::Error> {
        let tab_names = self.tab_names();
        self.terminal.draw(|frame| {
            let area = frame.size();
            //frame.render_widget(ratatui::widgets::Paragraph::new("Hello world"), area);
            frame.render_widget(self.navbar.render(tab_names), area);
            frame.set_cursor(0,0);
            // TBD Split frame, show navbar at bottom
        })?;

        Ok(())
    }
}

#[derive(Clone, Copy)]
pub enum CursorDirection { Up, Down, Left, Right }


pub struct TuiNavbar {
    pub show_tabs: bool,
    //pub show_bookmarks: bool,
} impl TuiNavbar {

    pub fn new() -> TuiNavbar {
        TuiNavbar { show_tabs: true }
    }

    pub fn render(&self, tab_names: Vec<String>) -> Tabs {
       Tabs::new(tab_names)
            .block(Block::default().title("Pages").borders(Borders::NONE))
            .style(Style::default().white().bg(Color::Blue))
            .highlight_style(Style::default().yellow())
            .select(0)
            .divider(symbols::DOT)
            .padding("┌", "┐") // TBD No padding for active tab, has top 3 sides border instead
    }
}

pub struct TuiTab {
    pub id: String,
    pub name: String,
    pub dom: TuiNode,
    pub layout: Layout
} impl TuiTab {

    pub fn new(id: &str) -> TuiTab {
        Self {
            id: id.to_string(),
            name: String::new(),
            dom: TuiNode::null(),
            layout: Layout::default()
        }
    }

    pub fn update(&mut self, dom: TuiNode) -> Result<(), anyhow::Error> {
        self.dom = dom;
        self.update_name(Some("test tab")); //None);
        self.update_layout();

        Ok(()) // TBD
    }

    pub fn update_name(&mut self, name: Option<&str>) {
        if let Some(name) = name { self.name = name.to_owned() }
        else {
            // TBD use dom to find <title> in <header> and extract to name field for easy reference
        }
    }

    pub fn update_layout(&mut self) {
        let layout = Layout::default();
        // TBD use dom to create layout?
        // Find Phantom nodes & collapse
        // Then when rendering, skip phantom nodes when calculating index
        self.layout = layout;
    }

}

pub struct TuiNode {
    pub children: Vec<TuiNode>
} impl TuiNode {
    fn null() -> TuiNode {
        TuiNode { children: vec![] }
    }
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
























