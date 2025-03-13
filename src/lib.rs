use std::collections::HashMap;

use blaze_explorer_lib::{mode::Mode, plugin::Plugin};
use git_diff_plugin::GitDiff;
use ratatui::crossterm::event::KeyEvent;

mod commands;
mod defaults;
mod git_diff_plugin;
mod popup;

//Plugin getter
#[unsafe(no_mangle)]
pub extern "Rust" fn get_plugin(
    bindings_map: HashMap<(Mode, Vec<KeyEvent>), String>,
) -> Box<dyn Plugin> {
    Box::new(GitDiff::new(bindings_map))
}
