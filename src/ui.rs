
use crate::Etymon;
use crate::config::CursorControls;


pub enum UIMode { View, Write }
impl Etymon {

    #[allow(non_snake_case)]
    pub fn handle_ui(&mut self) -> Result<(), anyhow::Error> {
        // TODO
        Ok(())
    }

}
//impl Etymon {
//    pub fn process_keys(&mut self, keys: Keys<std::io::Stdin>) -> Result<(), anyhow::Error> {
//        match self.mode {
//            UIMode::View  => self.process_view_keys(keys)?,
//            UIMode::Write => () //self.process_write_keys(keys)?,
//        }
//        Ok(())
//    }
//
//    pub fn process_view_keys(&mut self, keys: Keys<std::io::Stdin>) -> Result<(), anyhow::Error> {
//        for key in keys {
//            if let Ok(key) = key { match key {
//                Key::Backspace  => print!("backsp?"),
//                Key::Left       => print!("history ancestor"),
//                Key::Right      => print!("history youngest child"),
//                Key::Up         => print!("history younger sibling"),
//                Key::Down       => print!("history older sibling"),
//                Key::Home       => print!("jump page top"),
//                Key::End        => print!("jump page bottom"),
//                Key::PageUp     => print!("scroll up"),
//                Key::PageDown   => print!("scroll down"),
//                Key::BackTab    => print!("backtab?"),
//                Key::Delete     => print!("delete?"),
//                Key::Insert     => print!("insert?"),
//                //Key::F(f_u8)    => self.process_fn_keymap(f_u8)?,
//                Key::Char(ch)   => self.process_view_char(ch)?,
//                //Key::Alt(ch)    => self.process_alt_keymap(ch),
//                //Key::Ctrl(ch)   => self.process_ctrl_view_char(ch)?,
//                Key::Null       => print!("end of input?"),
//                Key::Esc        => print!("reset tui state without reloading"),
//                /* Invalid */ _ => print!("not an action yet"),
//            }}
//        };
//        Ok(()) // TBD Handle minor errors with logging reports
//    }
//
//    /// Handles char input, either as cursor for WASD/HJKL or to call configured bindings.
//    pub fn process_view_char(&mut self, ch: char) -> Result<(), anyhow::Error> {
//        match ch {
//            'w'|'a'|'s'|'d'|'h'|'j'|'k'|'l' => self.process_view_cursor(ch)?,
//            'q' => self.quit(), // TBD Final location where?
//            'p' => panic!("This panic is used to close the program! Toodaloo!"), // TBD Final location where?
//            _ => self.process_keymap(ch),
//        }
//        Ok(()) // TBD Handle minor errors with logging reports
//    }
//
//    /// Handles cursor movement based on CursorControls (WASD/HJKL).
//    pub fn process_view_cursor(&mut self, ch: char) -> Result<(), anyhow::Error> {
//        use crate::display::CursorDirection::*;
//        match crate::CONFIG.cursor_controls {
//            CursorControls::WASD => match ch {
//                'w' => self.display.move_cursor(Up)?,
//                'a' => self.display.move_cursor(Left)?,
//                's' => self.display.move_cursor(Down)?,
//                'd' => self.display.move_cursor(Right)?,
//                'h'|'j'|'k'|'l' => self.process_keymap(ch),
//                _ => panic!("Use keybinds for custom cursor instead of calling this function!"),
//            },
//            CursorControls::HJKL => match ch {
//                'h' => self.display.move_cursor(Left)?,
//                'j' => self.display.move_cursor(Down)?,
//                'k' => self.display.move_cursor(Up)?,
//                'l' => self.display.move_cursor(Right)?,
//                'w'|'a'|'s'|'d' => self.process_keymap(ch),
//                _ => panic!("Use keybinds for custom cursor instead of calling this function!"),
//            }
//        }
//        Ok(())
//    }
//
//    pub fn process_keymap(&mut self, ch: char) {
//        if let Some(mapped) = crate::CONFIG.char_keymap.binds.get(&ch) {
//            // TODO Do something with mapped
//            print!("bind {}: {}", ch, mapped)
//        }
//    }
//
//    /*
//    /// TBD Map to keybinds in configuration
//    pub fn process_alt_keymap(&mut self, ch: char) {
//        if let Some(mapped) = crate::CONFIG.alt_keymap.binds.get(&ch) {
//            // TODO Do something with mapped
//            print!("bind {}: {}", ch, mapped)
//        }
//    }
//
//    /// TBD Map to essential controls/cursors, then keybind config
//    pub fn process_ctrl_view_char(&mut self, ch: char) -> Result<(), anyhow::Error> {
//        match ch {
//            'q' => self.quit(),
//            _ => (),
//        }
//        Ok(())
//    }
//
//    pub fn process_fn_keymap(&mut self, f: u8) -> Result<(), anyhow::Error> {
//        // TODO try to find keybind in config hashmap, then execute
//        Ok(())
//    }
//
//
//
//
//
//
//
//    pub fn process_write_keys(&mut self, keys: Keys<std::io::Stdin>) -> Result<(), anyhow::Error> {
//     todo!{}
//    }
//    */
//}
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
