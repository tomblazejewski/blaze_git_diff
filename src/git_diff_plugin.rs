use blaze_explorer_lib::{action::Action, mode::Mode};
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
pub struct GitDiff {
    plugin_bindings: HashMap<(Mode, Vec<KeyEvent>), String>,
    popup_bindings: HashMap<(Mode, Vec<KeyEvent>), String>,
    functionality_map: HashMap<String, Action>,
}

impl GitDiff {
    pub fn new(bindings_map: HashMap<(Mode, Vec<KeyEvent>), String>) -> GitDiff {
        GitDiff {
            plugin_bindings: bindings_map,
            popup_bindings: HashMap::new(),
            functionality_map: HashMap::new(),
        }
    }
}
