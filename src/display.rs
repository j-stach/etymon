
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
    pub cursor: TuiCursor,
    pub needs_update: bool,
} impl Display {

    pub fn new() -> Result<Display, anyhow::Error> {
        let screen = std::io::stdout().into_raw_mode()?.into_alternate_screen()?;
        if let Ok(terminal) = Terminal::new(TermionBackend::new(screen)) {
            let display = Display {
                terminal,
                tabs: VecDeque::new(),
                navbar: TuiNavbar::new(),
                cursor: TuiCursor::new(),
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

    pub fn cursor(&self) -> &TuiCursor { &self.cursor }

    pub fn move_cursor(&mut self, direction: CursorDirection) {
        use CursorDirection::*;
        match direction {
            Up    => self.cursor.travel( 0, 1),
            Down  => self.cursor.travel( 0,-1),
            Left  => self.cursor.travel(-1, 0),
            Right => self.cursor.travel( 1, 0),
        }
    }

    pub fn fast_cursor(&mut self, direction: CursorDirection, speed: u16) { // TODO Test
        use CursorDirection::*;
        let speed = speed as i16;
        match direction {
            Up    => self.cursor.travel( 0, speed),
            Down  => self.cursor.travel( 0,-speed),
            Left  => self.cursor.travel(-speed, 0),
            Right => self.cursor.travel( speed, 0),
        }
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

pub struct TuiCursor {
    pub position: (u16, u16),
    pub max: (u16, u16)
} impl TuiCursor {

    pub fn new() -> TuiCursor {
        Self { position: (1, 1), max: (1, 1) }
    }

    pub fn check_x(&self, dest: u16) -> bool { dest <= self.max.0 }

    pub fn check_y(&self, dest: u16) -> bool { dest <= self.max.1 }

    pub fn travel(&mut self, x: i16, y: i16) {
        let xabs = x.abs() as u16;
        let yabs = y.abs() as u16;

        if x > 0 {
            if self.check_x(xabs + self.position.0) {
                termion::cursor::Right(xabs);
                self.position.0 += xabs;
            } else {
                termion::cursor::Right(self.position.0);
                self.position.0 = self.max.0;
            }
        } else if x < 0 {
            if self.check_x(self.position.0 - xabs) {
                termion::cursor::Left(xabs);
                self.position.0 -= xabs;
            } else {
                termion::cursor::Left(self.position.0);
                self.position.0 = 0;
            }
        }

        if y > 0 {
            if self.check_y(yabs + self.position.0) {
                termion::cursor::Up(yabs);
                self.position.0 += yabs;
            } else {
                termion::cursor::Up(self.position.0);
                self.position.0 = self.max.0;
            }
        } else if y < 0 {
            if self.check_y(self.position.0 - yabs) {
                termion::cursor::Down(yabs);
                self.position.0 -= yabs;
            } else {
                termion::cursor::Down(self.position.0);
                self.position.0 = 0;
            }
        }
    }

    pub fn move_to(&mut self, mut x: u16, mut y: u16) {
        if !self.check_x(x) { x = self.max.0 }
        if !self.check_x(y) { y = self.max.1 }

        termion::cursor::Goto(x + 1, y + 1); // TODO Test, is this what (1, 1)-based means?
        self.position = (x, y);
    }

}

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
























