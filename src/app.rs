use ratatui::{prelude::Backend, Frame, Terminal};

use crate::{errors::DoraResults, header::Header, input::{Control, InputHandler}, table_ui::{TableUI, TableUIState}};

pub struct App {
    // input_handler
    input_handler: InputHandler,
    // table_state
    table_state: TableUIState,
}

impl App {
    pub fn new() -> Self {
        Self {
            input_handler: InputHandler::new(),
            table_state: TableUIState::new(),
        }
    }

    pub fn main_loop<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> DoraResults<Option<String>> {
        loop {
            let control = self.input_handler.next();
            if matches!(control, Control::Quit) {
                return Ok(None)
            }

            self.step(&control);
            self.draw(terminal);
        }
    }

    fn step(&mut self, control: &Control) {
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let size = frame.area();

        let table_state = self.table_state;
        let df = table_state.dataframe;

        // get headers
        let df_schema = df.schema();
        let mut headers: Vec<Header> = vec![];
        for (col_name, dt) in df_schema.iter() {
            headers.push(
                Header{name: col_name.to_string()}
            );
        }

        // get rows
        
        let table = TableUI::new(headers, );
        frame.render_stateful_widget(table, size, &mut self.table_state);
    }

    fn draw<B: Backend>(&mut self, terminal: &mut Terminal<B>) {
        // let start = Instant::now();
        terminal.draw(|f| {
            self.render_frame(f);
        }).unwrap();

    }
}