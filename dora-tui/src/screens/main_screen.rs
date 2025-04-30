use std::sync::Arc;

use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind}, layout::{Constraint, Layout}, style::{Color, Modifier, Style, Stylize}, text::{Line, Span, Text}, widgets::{ListItem, Paragraph}, DefaultTerminal, Frame
};

use crate::app::{App, InputMode};

use super::traits::ScreenRenderer;

pub struct MainScreen {
}

impl MainScreen {
    pub fn new() -> Self {
        Self {}
    }
}

impl ScreenRenderer for MainScreen {
    fn draw(&self, frame: &mut Frame, app_state: &App) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ]);
        let [help_area, input_area, messages_area] = vertical.areas(frame.area());
        let (msg, style) = match app_state.input_mode {
            InputMode::Normal => (
                vec![
                    "Press ".into(),
                    "q".bold(),
                    " to exit, ".into(),
                    "e".bold(),
                    " to start editing.".bold(),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Editing => (
                vec![
                    "Press ".into(),
                    "Esc".bold(),
                    " to stop editing, ".into(),
                    "Enter".bold(),
                    " to record the message".into(),
                ],
                Style::default(),
            ),
        };

        let text = Text::from(Line::from(msg)).patch_style(style);
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, help_area);
    }
}