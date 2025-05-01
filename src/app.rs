use ratatui::{prelude::Backend, Terminal};

use crate::{errors::DoraResults, input::{Control, InputHandler}};

pub struct App {
    // input_handler
    input_handler: InputHandler
    // table_state
}

impl App {
    pub fn new() -> Self {
        Self {
            input_handler: InputHandler::new(),
        }
    }

    pub fn main_loop<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> DoraResults<Option<String>> {
        loop {
            let control = self.input_handler.next();
            if matches!(control, Control::Quit) {
                return Ok(None)
            }
        }
    }
}