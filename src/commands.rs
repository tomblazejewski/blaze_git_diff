use blaze_explorer_lib::{
    action::Action,
    app::App,
    plugin::plugin_helpers::{PluginFetchResult, access_plugin},
};

use crate::{defaults::PLUGIN_NAME, popup::GitDiffWindow};

pub fn open_diff_box(app: &mut App) -> Option<Action> {
    let result = access_plugin(app, PLUGIN_NAME);
    let plugin = match result {
        PluginFetchResult::Err(action) => return action,
        PluginFetchResult::Ok(plugin) => plugin,
    };
    let popup_keymap = plugin.get_popup_keymap();
    let popup = Box::new(GitDiffWindow::new(popup_keymap));
    app.attach_popup(popup);

    None
}
