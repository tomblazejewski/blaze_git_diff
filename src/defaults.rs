use std::collections::HashMap;

use crate::commands::open_diff_box;
use blaze_explorer_lib::input_machine::input_machine_helpers::convert_str_to_events;
use blaze_explorer_lib::insert_binding;
use blaze_explorer_lib::mode::Mode;
use blaze_explorer_lib::plugin::plugin_action::PluginAction;
use blaze_explorer_lib::{
    action::{Action, AppAction},
    create_plugin_action, custom_action,
    plugin::plugin_commands::PluginQuit,
};
use ratatui::crossterm::event::KeyEvent;

pub const PLUGIN_NAME: &str = "blaze_git_diff";

//Default functionalities
pub fn get_functionalities() -> HashMap<String, Action> {
    let mut functionality_map = HashMap::new();
    functionality_map.insert("GitDiffOpen".to_string(), custom_action!(open_diff_box));
    functionality_map.insert("GitDiffQuit".to_string(), create_plugin_action!(PluginQuit));

    functionality_map
}

//Default bindings
pub fn get_default_bindings() -> HashMap<(Mode, Vec<KeyEvent>), String> {
    let mut bindings_map = HashMap::new();
    insert_binding!(bindings_map, Mode::PopUp, "<Esc>", "GitDiffQuit");
    insert_binding!(bindings_map, Mode::Normal, "G", "GitDiffOpen");
    bindings_map
}
