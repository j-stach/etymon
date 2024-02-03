
use termion::{input::Keys, event::Key};
use crate::display::Display;
use crate::config::CursorControls;


pub enum UIMode { View, Write }

pub enum UIEvent {
    DisplayEvent,
        // moving cursor, reordering tabs, hghlight, copy/paste
    BrowserEvent
        // form submission, button click, relaod, navigate page
}

pub fn process_keys(display: &mut Display, mode: &UIMode, keys: Keys<std::io::Stdin>) -> Result<(), anyhow::Error> {
    match mode {
        UIMode::View  => process_view_keys(display, keys)?,
        UIMode::Write => process_write_keys(display, keys)?,
    }
        // termion wait for ui event
        // if display event, execute and re-render
        // if browser event, pass event to javascript & mark for update

    Ok(())
}

pub fn process_view_keys(display: &mut Display, keys: Keys<std::io::Stdin>) -> Result<(), anyhow::Error> {
    for key in keys {
        if let Ok(key) = key { match key {
            Key::Backspace  => print!("backsp?"),
            Key::Left       => print!("history ancestor"),
            Key::Right      => print!("history youngest child"),
            Key::Up         => print!("history younger sibling"),
            Key::Down       => print!("history older sibling"),
            Key::Home       => print!("jump page top"),
            Key::End        => print!("jump page bottom"),
            Key::PageUp     => print!("scroll up"),
            Key::PageDown   => print!("scroll down"),
            Key::BackTab    => print!("backtab?"),
            Key::Delete     => print!("delete?"),
            Key::Insert     => print!("insert?"),
            Key::F(f_u8)    => process_fn_keymap(f_u8, display)?,
            Key::Char(ch)   => process_view_char(ch, display)?,
            Key::Alt(ch)    => process_alt_view_char(ch, display)?,
            Key::Ctrl(ch)   => process_ctrl_view_char(ch, display)?,
            Key::Null       => print!("end of input?"),
            Key::Esc        => print!("reset tui state without reloading"),
            /* Invalid */ _ => print!("not an action"),
        }}
    };
    todo!{}
}

pub fn process_fn_keymap(f: u8, display: &mut Display) -> Result<(), anyhow::Error> {
    // TODO try to find keybind in config hashmap, then execute
    todo!{}
}

/// TBD Map to essential controls/cursors, then keybind config
pub fn process_view_char(ch: char, display: &mut Display) -> Result<(), anyhow::Error> {
    match ch {
        'w'|'a'|'s'|'d'|'h'|'j'|'k'|'l' => process_cursor(ch, display)?,
        _ => process_keybind(ch, display),
    }
    Ok(())
}

pub fn process_cursor(ch: char, display: &mut Display) -> Result<(), anyhow::Error> {
    use crate::display::CursorDirection::*;
    match crate::CONFIG.cursor_controls {
        CursorControls::WASD => match ch {
            'w' => display.move_cursor(Up),
            'a' => display.move_cursor(Left),
            's' => display.move_cursor(Down),
            'd' => display.move_cursor(Right),
            'h'|'j'|'k'|'l' => process_keybind(ch, display),
            _ => panic!("Use keybinds for custom cursor instead of calling this function!"),
        },
        CursorControls::HJKL => match ch {
            'h' => display.move_cursor(Left),
            'j' => display.move_cursor(Down),
            'k' => display.move_cursor(Up),
            'l' => display.move_cursor(Right),
            'w'|'a'|'s'|'d' => process_keybind(ch, display),
            _ => panic!("Use keybinds for custom cursor instead of calling this function!"),
        }
    }
    Ok(())
}

pub fn process_keybind(ch: char, display: &Display) {
    // TODO try to find keybind in config hashmap, then execute
    todo!{}
}

/// TBD Map to keybinds in configuration
pub fn process_alt_view_char(ch: char, display: &Display) -> Result<(), anyhow::Error> {
    match ch {
        _ => ()
    }
    Ok(())
}

/// TBD Map to essential controls/cursors, then keybind config
pub fn process_ctrl_view_char(ch: char, display: &Display) -> Result<(), anyhow::Error> {
    match ch {
        _ => ()
    }
    Ok(())
}







pub fn process_write_keys(display: &Display, keys: Keys<std::io::Stdin>) -> Result<(), anyhow::Error> {
 todo!{}
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
