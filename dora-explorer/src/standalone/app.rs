use std::error::Error;

use ratatui::{
    Frame, Terminal,
    layout::{Constraint, Layout},
    prelude::Backend,
    style::{Color, Stylize},
    widgets::Paragraph,
};

use crate::library::{control::Control, Controller, ExplorerState, ExplorerUI};

// global app state.
pub struct App {
    // global states (regardless of page)
    pub explorer_state: ExplorerState,
}

impl App {
    pub fn new(file_path: Option<String>) -> Self {
        Self {
            explorer_state: ExplorerState::new(file_path),
        }
    }

    pub fn main_loop<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            let control = self.explorer_state.input_handler.next();
            if self.explorer_state.should_exit() {
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
        let ui = ExplorerUI::new();
        frame.render_stateful_widget(ui, frame.area(), &mut self.explorer_state);
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
