
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect}, style::{Modifier, Style, Stylize}, text::{Line, Text}, widgets::{Block, Borders, Paragraph, Wrap}, Frame
};

use crate::app::{App, InputMode};

use super::traits::ScreenRenderer;

pub struct MainScreen {
}

static WELCOME_MESSAGE: &str = "HELLO";

impl ScreenRenderer for MainScreen {
    fn draw(&self, frame: &mut Frame, app_state: &App) {
        let [_banner_top, v_main, banner_bottom] = MainScreen::vbox_with_top_and_bottom_banners(frame);
        
        MainScreen::draw_bottom_banner(frame, app_state, &banner_bottom);

        let [_pad_left, h_main, _pad_right] = MainScreen::hbox_with_left_and_right_padding(&v_main);

        MainScreen::draw_boxed_centred_message(frame, &h_main);
    }

}



impl MainScreen {
    pub fn new() -> Self {
        Self {}
    }

    fn vbox_with_top_and_bottom_banners(frame: &mut Frame) -> [Rect;3] {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(2),
            Constraint::Length(1),
        ]);
        let result = vertical.areas(frame.area());
        return result;
    }

    fn hbox_with_left_and_right_padding(vertical_area: &Rect) -> [Rect;3] {
        let horizontal = Layout::horizontal([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ]);
        // rendering main block in center
        let result = horizontal.areas(*vertical_area);
        return result;
    }
    
    
    fn draw_bottom_banner(frame: &mut Frame, app_state: &App, area: &Rect) {
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
        let banner_message = Paragraph::new(text);
        frame.render_widget(banner_message, *area);
    }
    
    fn draw_boxed_centred_message(frame: &mut Frame, area: &Rect) {
        let text_lines = WELCOME_MESSAGE.lines().count() as u16;
        // Calculate top padding for vertical centering (or adjust for top/bottom alignment)
        let padding_top = (area.height.saturating_sub(text_lines)) / 2;
        let padding_bottom = area.height.saturating_sub(text_lines) - padding_top;

        let verticals = Layout::vertical([
            Constraint::Length(padding_top),
            Constraint::Min(text_lines),
            Constraint::Length(padding_bottom),
        ]);
        let [
            _,
            center_area,
            _,
        ] = verticals.areas(*area);
        
        let block = Block::new()
            .borders(Borders::ALL);
        let para = Paragraph::new(WELCOME_MESSAGE)
            .wrap(Wrap{ trim: true })
            .alignment(Alignment::Center);
        frame.render_widget(
            block,
            *area,
        );
        frame.render_widget(
            para,
            center_area,
        );
    }
}


