
use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use super::browser::NodeDataKind::{self, *};

/*
    TODO
    - Setup VPN connection through browser?
    - Backend selector Chrome/TOR
*/

lazy_static::lazy_static!{
    /// Holds the configuration values for Etymon & its children.
    pub static ref CONFIG: ConfigOptions = ConfigOptions::read_config();
}

#[derive(Serialize, Deserialize)]
pub struct ConfigOptions {
    /// Webpage url to open on startup. Needs to begin with https://...
    pub homepage: String,
    /// If true, links to an existing user Chrome/Chromium profile. Default is false.
    pub sync_profile: bool,
    /// If true, Etymon does not save session data or logs. Default is true.
    pub amnesia: bool,
    /// Useful document structures to render to TUI.
    pub useful_nodes: HashSet<NodeDataKind>,
    /// Use WASD or HJKL for View-mode cursor control.
    pub cursor_controls: CursorControls,
    /// Function keymaps call external scripts by path.
    pub fn_keymap: FnKeymap,
    /// Character keymaps work in View mode.
    pub char_keymap: CharKeymap,
    /// Alt + characters work in both View and Edit mode.
    pub alt_keymap: CharKeymap,
    /// UI polling frequency (per second).
    pub tick_rate: f32,
    /// Screen refresh frequency (per second).
    pub frame_rate: f32,
    /// Starts Edymon with mouse events enabled, if terminal supports them.
    pub mouse_capture: bool,
    // TBD Font size
} impl ConfigOptions {

    // TBD Read & parse config.toml
    fn read_config() -> ConfigOptions {
        Self::default_options()
    }

    /// Private, minimal, and performant configuration with no bindings.
    fn default_options() -> ConfigOptions {
        ConfigOptions {
            homepage: "https://www.duckduckgo.com".to_owned(),
            sync_profile: false,
            amnesia: true,
            useful_nodes: [Element, Text].into_iter().collect(),
            cursor_controls: CursorControls::WASD,
            // TODO Currently unused:
            fn_keymap: FnKeymap::default(),
            char_keymap: CharKeymap::default(),
            alt_keymap: CharKeymap::default(),
            tick_rate: 4.0,
            frame_rate: 60.0,
            mouse_capture: false,
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum CursorControls { WASD, HJKL }

#[derive(Default, Serialize, Deserialize)]
pub struct CharKeymap {
    pub binds: std::collections::HashMap<char, String>
}

/// Fn key mappings are for static functions that take Etymon itself as the only argument.
#[derive(Default, Serialize, Deserialize)]
pub struct FnKeymap {
    pub scripts: std::collections::HashMap<usize, FnScript>
} impl FnKeymap {
    pub fn serve_fn(&self, id: usize) -> Result<(), anyhow::Error> {
        todo!{"Match id to fn key"}
    }
}

#[derive(Serialize, Deserialize)]
/// Specify the executor to call for the mapped script. Easier for everyone this way.
pub enum FnScript {
    None,
    Rust(()/* TBD Function Pointer? */),
    Perl(String), // TBD Filepath type?
    Lua(String),  // TBD Filepath type?
} impl FnScript {

    pub fn execute(&self) {
        match self {
            FnScript::None        => (),
            FnScript::Rust(rust)  => FnScript::execute_rust(*rust),
            FnScript::Perl(perl)  => FnScript::execute_perl(perl),
            FnScript::Lua(lua)    => FnScript::execute_lua(lua),
        }
    }

    pub fn execute_rust(rust: ()) {
        todo!{"Execute rust functions from library on page, text, and browser"}
    }

    pub fn execute_perl(script_path: &str) {
        todo!{"Execute perl scripts on page, text, and browser"}
    }

    pub fn execute_lua(script_path: &str) {
        todo!{"Execute lua scripts on page, text, and browser"}
    }
}


impl std::default::Default for FnScript {
    fn default() -> Self {
        FnScript::None
    }
}


