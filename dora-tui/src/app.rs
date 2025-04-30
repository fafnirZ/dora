
use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind}, 
    layout::{Constraint, Layout}, style::{Color, Modifier, Style, Stylize}, 
    text::{Line, Span, Text}, widgets::{ListItem, Paragraph}, DefaultTerminal, Frame
};

use crate::screens::{main_screen::MainScreen, traits::ScreenRenderer};

/// App holds the state of the application
pub struct App {
    /// Current input mode
    pub input_mode: InputMode,
    pub curr_screen: Box<dyn ScreenRenderer>, // more interface like, instead of using enums
}

pub enum InputMode {
    Normal,
    Editing,
}
pub enum Screen {
    MainScreen,
}

impl App {
    pub fn new() -> Self {
        Self {
            input_mode: InputMode::Normal,
            curr_screen: Box::new(MainScreen::new()),
        }
    }
    // main loop
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| 
                self.curr_screen.draw(frame, &self)
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

    pub fn set_curr_screen(mut self, new_screen: Screen) {
        match new_screen {
            Screen::MainScreen => {
                self.curr_screen = Box::new(MainScreen::new());
            },
        }
    }
}
