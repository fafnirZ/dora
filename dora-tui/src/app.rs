
use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind}, layout::{Constraint, Layout}, style::{Color, Modifier, Style, Stylize}, text::{Line, Span, Text}, widgets::{ListItem, Paragraph}, DefaultTerminal, Frame
};

/// App holds the state of the application
pub struct App {
    /// Current input mode
    input_mode: InputMode,
}

enum InputMode {
    Normal,
    Editing,
}

impl App {
    pub const fn new() -> Self {
        Self {
            input_mode: InputMode::Normal,
        }
    }
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
            loop {
                terminal.draw(|frame| self.draw(frame))?;

                if let Event::Key(key) = event::read()? {
                    match self.input_mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Char('i') => {
                                self.input_mode = InputMode::Editing;
                            }
                            KeyCode::Char('q') => {
                                return Ok(());
                            }
                            _ => {}
                        },
                        InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                            KeyCode::Esc => self.input_mode = InputMode::Normal,
                            _ => {}
                        },
                        InputMode::Editing => {}
                    }
                }
            }
        }

    pub fn draw(&self, frame: &mut Frame) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ]);
        let [help_area, input_area, messages_area] = vertical.areas(frame.area());
        let (msg, style) = match self.input_mode {
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
