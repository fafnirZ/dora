
use ratatui::{layout::{Constraint, Layout}, prelude::Backend, Frame, Terminal};

use crate::{errors::DoraResults, input::{Control, InputHandler}, mode_banner::{self, AppModeState, ModeBanner}, table_ui::{TableUI, TableUIState}};

// global app state.
pub struct App {
    // input_handler
    input_handler: InputHandler,
    // table_state
    table_state: TableUIState,
    // mode banner
    mode_state: AppModeState,
}

impl App {
    pub fn new() -> Self {
        Self {
            input_handler: InputHandler::new(),
            table_state: TableUIState::new(),
            mode_state: AppModeState::new(),
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
        let area = frame.area();
        let [
            top_banner,
            main_area,
            bottom_banner,
        ] = Layout::vertical([
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ]).areas(area);
        let table = TableUI::new();
        let mode_banner = ModeBanner::new();
        frame.render_stateful_widget(table, main_area, &mut self.table_state);
        frame.render_stateful_widget(mode_banner, bottom_banner, &mut self.mode_state);
    }

    fn draw<B: Backend>(&mut self, terminal: &mut Terminal<B>) {
        // let start = Instant::now();
        terminal.draw(|f| {
            self.render_frame(f);
        }).unwrap();
    }
}