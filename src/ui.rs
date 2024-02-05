
use crate::Etymon;
use crate::config::CursorControls;

use crossterm::event::{read, Event, KeyEvent, KeyCode::{self, *}, KeyEventKind, KeyModifiers};

/// UIMode describes the various contexts for input events.
/// Fn keys work in all modes, but keymaps only work in View,
/// because in Insert mode characters keys are used to place text.
pub enum UIMode {
    /// Default mode for navigating pages and viewing text.
    View,
    /// Mode for the insertion of character into forms or other test-based widgets.
    Insert
}
impl Etymon {

    #[allow(non_snake_case)]
    pub fn handle_ui(&mut self) -> Result<(), anyhow::Error> { // TODO Replace with async for simultaneous keys
        use UIMode::*;
        let event = read()?;
        loop {
            // TBD Timeout?
            match event {
                Event::FocusGained => {/* TBD */},
                Event::FocusLost => {/* TBD */},
                Event::Key(event) => match self.mode {
                    View => self.view_key_event(event)?,
                    Insert => self.insert_key_event(event)?
                },
                Event::Mouse(_event) => {/* TBD */},
                Event::Paste(ref _data) => {/* TBD */},
                Event::Resize(_width, _height) => {/* TBD */},
            }
        }
    }

    /// Process keyboard events in View mode.
    pub fn view_key_event(&mut self, event: KeyEvent) -> Result<(), anyhow::Error> {

        let mods = |mods| { event.modifiers.contains(mods) };

        let none  = KeyModifiers::from_name("NONE").unwrap();
        let shift = KeyModifiers::from_name("SHIFT").unwrap();
        let alt   = KeyModifiers::from_name("ALT").unwrap();
        let ctrl  = KeyModifiers::from_name("CONTROL").unwrap();
        let ctrl_shift = { let mut m = shift.clone(); m.insert(ctrl.clone()); m };
        let ctrl_alt   = { let mut m = alt.clone(); m.insert(ctrl.clone()); m };
        let alt_shift  = { let mut m = shift.clone(); m.insert(alt.clone()); m };
        let maximum_overmod = { let mut mo = ctrl_shift.clone(); mo.insert(alt.clone()); mo };

        match event.code {

            Char(_)              => {
                if mods(none)               { self.handle_char(event)? }
                else if mods(maximum_overmod) { todo!{"OMG"}}
                else if mods(ctrl_shift)    { todo!{"ETYMON/SYSTEM RESERVED"}}
                else if mods(ctrl_alt)      { todo!{"Alternate control keymap"}}
                else if mods(alt_shift)     { todo!{"Alternate shifted keymap"}}
                else if mods(ctrl)          { todo!{"Control keymap"}}
                else if mods(shift)         { todo!{"Shifted keymap"}}
                else if mods(alt)           { todo!{"Alternate keymap"}}
            },

            Enter                 => {
                if mods(none) { todo!{"Select a button or form"}}
                else {/* TBD */}
            },

            _ => (),

            // TBD Unimplemented:
            /*
            Esc                   => println!("End subroutine and return to base view mode"),
            Left                  => println!("Move focus to left widget"),
            Right                 => println!("Widget to right"),
            Up                    => println!("Widget above"),
            Down                  => println!("Widget below"),
            Home                  => println!("Jump to page top"),
            End                   => println!("Jump to page bottom"),
            PageUp                => println!("Scroll up"),
            PageDown              => println!("Scroll down"),
            F(f_u8)               => println!("Execute mapped script on current page"),

            Backspace              => println!("reconstruct dom, removing user changes"),
            Tab                    => println!("cycle through forms"),
            BackTab                => println!("cycle through forms in reverse"),
            Delete                 => println!("remove tui element from dom"),
            Insert                 => println!("enter html edit mode"),
            PrintScreen            => println!("take snapshot of tui and/or chrome as pdf"),
            CapsLock               => (),
            ScrollLock             => (),
            NumLock                => (),
            Pause                  => (),
            Menu                   => (),
            KeypadBegin            => (),
            Null                   => (),
            Media(_media_key_code) => (),
            Modifier(_modifier)    => (),
            */
        }
        Ok(())
    }

    /// Handles View-mode KeyCode::Char event depending on the type (press or hold).
    pub fn handle_char(&mut self, event: KeyEvent) -> Result<(), anyhow::Error> {
        let ch = match event.code {
            KeyCode::Char(ch) => ch,
            _ => panic!("Should have passed this function a KeyCode::Char event!")
        };
        use KeyEventKind::*;
        match event.kind {
            Press => self.char_press(ch)?,
            Repeat => self.char_hold(ch)?,
            Release => {/* TBD */},
        }
        Ok(())
    }

    /// Handles character key presses in a View mode environment.
    pub fn char_press(&mut self, ch: char) -> Result<(), anyhow::Error> {
        match ch {
            'w'|'a'|'s'|'d'|'h'|'j'|'k'|'l' => self.process_cursor(ch),
            'q' => Ok(self.quit()), // TBD Final location where?
            'p' => panic!("This panic was used to intentionally crash the program! Toodaloo!"), // TBD Final location where?
            _ => Ok(self.process_keymap(ch)),
        }
    }

    /// Only cursor movement keys repeat their action when held.
    pub fn char_hold(&mut self, ch: char) -> Result<(), anyhow::Error> {
        match ch {
            'w'|'a'|'s'|'d'|'h'|'j'|'k'|'l' => self.process_hold_cursor(ch)?,
            _ => {/* TBD */},
        }
        Ok(())
    }

    /// Handles cursor movement based on CursorControls (WASD/HJKL).
    pub fn process_cursor(&mut self, ch: char) -> Result<(), anyhow::Error> {
        use crate::tui::CursorDirection::*;
        match crate::CONFIG.cursor_controls {
            CursorControls::WASD => match ch {
                'w' => self.tui.move_cursor(Up)?,
                'a' => self.tui.move_cursor(Left)?,
                's' => self.tui.move_cursor(Down)?,
                'd' => self.tui.move_cursor(Right)?,
                'h'|'j'|'k'|'l' => self.process_keymap(ch),
                _ => panic!("Use keybinds for custom cursor instead of calling this function!"),
            },
            CursorControls::HJKL => match ch {
                'h' => self.tui.move_cursor(Left)?,
                'j' => self.tui.move_cursor(Down)?,
                'k' => self.tui.move_cursor(Up)?,
                'l' => self.tui.move_cursor(Right)?,
                'w'|'a'|'s'|'d' => self.process_keymap(ch),
                _ => panic!("Use keybinds for custom cursor instead of calling this function!"),
            }
        }
        Ok(())
    }

    /// Handles cursor movement when the direction key is held.
    pub fn process_hold_cursor(&mut self, ch: char) -> Result<(), anyhow::Error> {
        use crate::tui::CursorDirection::*;
        match crate::CONFIG.cursor_controls {
            CursorControls::WASD => match ch {
                'w' => self.tui.move_cursor(Up)?,
                'a' => self.tui.move_cursor(Left)?,
                's' => self.tui.move_cursor(Down)?,
                'd' => self.tui.move_cursor(Right)?,
                _ => {/* TBD */},
            },
            CursorControls::HJKL => match ch {
                'h' => self.tui.move_cursor(Left)?,
                'j' => self.tui.move_cursor(Down)?,
                'k' => self.tui.move_cursor(Up)?,
                'l' => self.tui.move_cursor(Right)?,
                _ => {/* TBD */},
            }
        }
        Ok(())
    }

    pub fn process_keymap(&mut self, ch: char) {
        if let Some(mapped) = crate::CONFIG.char_keymap.binds.get(&ch) {
            // TODO Do something with mapped
            print!("bind {}: {}", ch, mapped)
        }
    }

    /// Process keyboard events in View mode.
    pub fn insert_key_event(&mut self, event: KeyEvent) -> Result<(), anyhow::Error> {

        let mods = |mods| { event.modifiers.contains(mods) };

        let none  = KeyModifiers::from_name("NONE").unwrap();
        let shift = KeyModifiers::from_name("SHIFT").unwrap();
        let alt   = KeyModifiers::from_name("ALT").unwrap();
        let ctrl  = KeyModifiers::from_name("CONTROL").unwrap();
        let ctrl_shift = { let mut m = shift.clone(); m.insert(ctrl.clone()); m };
        let _ctrl_alt   = { let mut m = alt.clone(); m.insert(ctrl.clone()); m };
        let _alt_shift  = { let mut m = shift.clone(); m.insert(alt.clone()); m };
        let _maximum_overmod = { let mut mo = ctrl_shift.clone(); mo.insert(alt.clone()); mo };

        match event.code {

            Char(_) => {
                /* TODO self.insert_text() */
            },

            Enter => {
                if mods(none) { todo!{"Submit a form"}}
                else {/* TBD: Newline without CR */}
            },

            Esc                   => println!("Exit insert mode"),
            Left                  => println!("Move cursor to left within widget"),
            Right                 => println!("Widget to right within widget"),
            Up                    => println!("Move cursor up within widget"),
            Down                  => println!("Widget down within widget"),

            _ => (),

            // TBD Unimplemented:
            /*
            CapsLock               => (),
            Backspace              => println!("reconstruct dom, removing user changes"),
            PrintScreen            => println!("take snapshot of tui and/or chrome as pdf"),
            F(f_u8)               => println!("Execute mapped script on current page"),
            */
        }
        Ok(())
    }

}




//TBD
// Reserved keys
// depending on cursor mode, wasd or hjkl
// directional keys are used for navigating history branches: "HissTree"
// > last_child, < parent, ^ sibling more recent, v less recent


// keybinds: non-reserved are customizable in config dir
// lua scripts for keybinds?
// perl scripts for navigation & manipulation?

// navbar commands
// http://www... loads link via chrome
// :r http://... loads link or API request via reqwuest
// :c http://... same thing but with curl
// :w http://... again for wget
// :s keywords   searches web using default search engine
// :f keywords   searches current text

// :perl my $perl;
// :perl perlscript.pl

// :lua some_lua_code(); \
//       on_multiple_lines();
// :lua luascript.lua args

// :b   bookmarks table/menu view
// :h   history tree view
// :c   etymon settings/config editor
