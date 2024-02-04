
use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use super::browser::NodeDataKind::{self, *};

/*
    TODO
    - Setup VPN connection through browser?
    - Backend selector Chrome/TOR
*/

lazy_static::lazy_static!{
    /// Holds the configuration state for Etymon.
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
    /// Function keymaps call external scripts.
    pub fn_keymap: FnKeymap,
    /// Character keymaps work in View mode.
    pub char_keymap: CharKeymap,
    /// Alt + characters work in View and Edit mode.
    pub alt_keymap: CharKeymap,
    /// UI polling frequency (per second).
    pub tick_rate: f32,
    /// Screen refresh frequency (per second).
    pub frame_rate: f32,
    // TBD Font size
} impl ConfigOptions {

    // TBD Read & parse config.toml
    fn read_config() -> ConfigOptions {
        Self::default_options()
    }

    fn default_options() -> ConfigOptions {
        ConfigOptions {
            homepage: "https://www.duckduckgo.com".to_owned(),
            sync_profile: false,
            amnesia: true,
            useful_nodes: [Element, Text].into_iter().collect(),
            cursor_controls: CursorControls::WASD,
            fn_keymap: FnKeymap::default(),
            char_keymap: CharKeymap::default(),
            alt_keymap: CharKeymap::default(),
            tick_rate: 4.0,
            frame_rate: 60.0,
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum CursorControls { WASD, HJKL }

/// Fn key mappings are for static functions that take Etymon itself as the only argument.
#[derive(Default, Serialize, Deserialize)]
pub struct FnKeymap {   // TBD Needs type for script/ function to exec
    pub fn1:  Option<String>,
    pub fn2:  Option<String>,
    pub fn3:  Option<String>,
    pub fn4:  Option<String>,
    pub fn5:  Option<String>,
    pub fn6:  Option<String>,
    pub fn7:  Option<String>,
    pub fn8:  Option<String>,
    pub fn9:  Option<String>,
    pub fn10: Option<String>,
    pub fn11: Option<String>,
    pub fn12: Option<String>,
} impl FnKeymap {
    pub fn serve_fn() {}
    pub fn execute_rust() {}
    pub fn execute_perl() {}
    pub fn execute_lua() {}
}

#[derive(Default, Serialize, Deserialize)]
pub struct CharKeymap {
    pub binds: std::collections::HashMap<char, String>
}


