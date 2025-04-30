
use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind}, 
    layout::{Constraint, Layout}, style::{Color, Modifier, Style, Stylize}, 
    text::{Line, Span, Text}, widgets::{ListItem, Paragraph}, DefaultTerminal, Frame
};
use crate::screens::traits::Screen;

/// App holds the state of the application
pub struct App {
    /// Current input mode
    pub input_mode: InputMode,
    pub curr_screen: Screen,
}

pub enum InputMode {
    Normal,
    Editing,
}


impl App {
    pub const fn new() -> Self {
        Self {
            input_mode: InputMode::Normal,
            curr_screen: Screen::MainScreen,
        }
    }
    // main loop
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| 
                self.curr_screen.draw(frame, &mut self)
            )?;

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


}
