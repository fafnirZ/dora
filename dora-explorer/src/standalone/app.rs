use std::error::Error;

use ratatui::{
    Frame, Terminal,
    layout::{Constraint, Layout},
    prelude::Backend,
    style::{Color, Stylize},
    widgets::Paragraph,
};

use super::{
    input::{InputHandler},
};
use crate::library::{control::Control, Controller, ExplorerState};

// global app state.
pub struct App {
    // global states (regardless of page)
    pub input_handler: InputHandler,
    explorer_state: ExplorerState,
}

impl App {
    pub fn new() -> Self {
        Self {
            input_handler: InputHandler::new(),
            explorer_state: ExplorerState::new(),
        }
    }

    pub fn main_loop<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            let control = self.input_handler.next();
            if matches!(control, Control::Quit) {
                return Ok(());
            }

            self.step(&control);
            self.draw(terminal);
        }
    }

    // the primary function for handling state conditionals
    // and updating state.
    fn step(&mut self, control: &Control) {
        Controller::perform_actions(control, &mut self.explorer_state);
    }

    ///////////////
    // rendering //
    ///////////////

    fn render_frame(&mut self, frame: &mut Frame) {
        //
    }
   

    fn draw<B: Backend>(&mut self, terminal: &mut Terminal<B>) {
        // let start = Instant::now();
        terminal
            .draw(|f| {
                self.render_frame(f);
            })
            .unwrap();
    }
}
