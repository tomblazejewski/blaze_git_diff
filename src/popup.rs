use std::collections::HashMap;

use blaze_explorer_lib::{
    action::Action, mode::Mode, plugin::plugin_popup::PluginPopUp, tools::center_rect,
};
use color_eyre::eyre::Result;
use ratatui::{
    crossterm::event::KeyEvent,
    layout::{Constraint, Rect},
    style::{Style, palette::tailwind},
    text::{Line, Text},
    widgets::{Block, Borders, Clear, Paragraph},
};

fn format_line(line: &str) -> Line {
    Line::from(line).style(Style::default().fg(match line.chars().next().unwrap() {
        '+' => tailwind::GREEN.c200,
        '-' => tailwind::RED.c200,
        _ => tailwind::WHITE,
    }))
}

#[derive(Debug, Clone, PartialEq)]
pub struct GitDiffWindow {
    keymap: HashMap<(Mode, Vec<KeyEvent>), Action>,
    should_quit: bool,
    output: String,
}

impl GitDiffWindow {
    pub fn new(keymap: HashMap<(Mode, Vec<KeyEvent>), Action>) -> GitDiffWindow {
        GitDiffWindow {
            keymap,
            should_quit: false,
            output: "".to_string(),
        }
    }

    pub fn attach_output(&mut self, message: String) {
        self.output = message;
    }
}

impl PluginPopUp for GitDiffWindow {
    fn draw(&mut self, frame: &mut ratatui::Frame, area: Rect) -> Result<()> {
        let area = center_rect(area, Constraint::Percentage(80), Constraint::Percentage(80));
        frame.render_widget(Clear, area);
        let lines = self.output.lines().map(format_line).collect::<Vec<Line>>();
        let paragraph = Paragraph::new(Text::from(lines));
        let paragraph = paragraph.block(
            Block::default()
                .borders(Borders::ALL)
                .title(self.display_details()),
        );
        frame.render_widget(paragraph, area);
        Ok(())
    }

    fn push_search_char(&mut self, ch: char) -> Option<Action> {
        None
    }

    fn drop_search_char(&mut self) -> Option<Action> {
        None
    }

    fn quit(&mut self) {
        self.should_quit = true
    }

    fn should_quit(&self) -> bool {
        self.should_quit
    }

    fn erase_text(&mut self) -> Option<Action> {
        None
    }

    fn get_search_query(&self) -> String {
        "".to_string()
    }

    fn display_details(&self) -> String {
        "Git Diff".to_string()
    }

    fn get_own_keymap(&self) -> std::collections::HashMap<(Mode, Vec<KeyEvent>), Action> {
        self.keymap.clone()
    }

    fn confirm_result(&mut self) -> Option<Action> {
        None
    }

    fn next_result(&mut self) -> Option<Action> {
        None
    }

    fn previous_result(&mut self) -> Option<Action> {
        None
    }

    fn update_search_query(&mut self, _query: String) -> Option<Action> {
        None
    }

    fn destruct(&self) -> Option<Box<dyn blaze_explorer_lib::command::Command>> {
        None
    }

    fn context(&self) -> String {
        String::new()
    }

    fn get_default_action(&self) -> Box<fn(KeyEvent) -> Option<Action>> {
        Box::new(blaze_explorer_lib::app_input_machine::get_none_action)
    }

    fn update_app(&mut self, _app: &mut blaze_explorer_lib::app::App) {}
}
