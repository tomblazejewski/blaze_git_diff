use blaze_explorer_lib::{action::Action, construct_plugin, mode::Mode, plugin::Plugin};
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use std::collections::HashMap;

use crate::defaults::{PLUGIN_NAME, get_default_bindings, get_functionalities};

#[derive(PartialEq, Clone, Debug)]
pub struct GitDiff {
    plugin_bindings: HashMap<(Mode, Vec<KeyEvent>), String>,
    popup_bindings: HashMap<(Mode, Vec<KeyEvent>), String>,
    functionality_map: HashMap<String, Action>,
}

impl GitDiff {
    pub fn new(custom_bindings_map: HashMap<(Mode, Vec<KeyEvent>), String>) -> Self {
        construct_plugin!(
            get_functionalities,
            get_default_bindings,
            custom_bindings_map
        )
    }
}

impl Plugin for GitDiff {
    fn display_details(&self) -> String {
        PLUGIN_NAME.to_string()
    }
    fn get_plugin_bindings(&self) -> HashMap<(Mode, Vec<KeyEvent>), String> {
        self.plugin_bindings.clone()
    }

    fn get_popup_bindings(&self) -> HashMap<(Mode, Vec<KeyEvent>), String> {
        self.popup_bindings.clone()
    }

    fn get_functionality_map(&self) -> HashMap<String, Action> {
        self.functionality_map.clone()
    }
}
