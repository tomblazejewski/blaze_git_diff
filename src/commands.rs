use blaze_explorer_lib::{
    action::{Action, AppAction},
    app::App,
    command::DisplayMessage,
    plugin::plugin_helpers::{PluginFetchResult, access_plugin},
};
use std::{io, process::Command as ProcessCommand};

use crate::{defaults::PLUGIN_NAME, popup::GitDiffWindow};
fn get_diff_message(file_name: String) -> io::Result<String> {
    let output = ProcessCommand::new("git")
        .args(["diff", file_name.as_str()])
        .output();
    match output {
        Ok(output) => {
            let stdout = String::from_utf8(output.stdout).unwrap();
            let stderr = String::from_utf8(output.stderr).unwrap();
            let output = format!("{}{}", stdout, stderr);
            Ok(output)
        }
        Err(err) => Err(err),
    }
}

pub fn open_diff_box(app: &mut App) -> Option<Action> {
    let result = access_plugin(app, PLUGIN_NAME);
    let plugin = match result {
        PluginFetchResult::Err(action) => return action,
        PluginFetchResult::Ok(plugin) => plugin,
    };
    let file_name = app.explorer_manager.get_selected_string();
    if file_name.is_none() {
        return Some(Action::AppAct(AppAction::DisplayMessage(
            "No file selected".to_string(),
        )));
    }
    let file_name = file_name.unwrap();
    let popup_keymap = plugin.get_popup_keymap();
    let mut popup = Box::new(GitDiffWindow::new(popup_keymap));

    let output = get_diff_message(file_name.clone());
    match output {
        Ok(output) => {
            popup.attach_output(output);
        }
        Err(err) => return Some(Action::AppAct(AppAction::DisplayMessage(err.to_string()))),
    }
    app.attach_popup(popup);

    None
}
