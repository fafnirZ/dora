
use ratatui::{layout::{Constraint, Layout}, prelude::Backend, Frame, Terminal};

use crate::{config::ConfigState, controller::Controller, df::state::DataFrameState, errors::DoraResults, input::{self, Control, InputHandler}, mode_banner::ModeBanner, search::state::SearchResultState, table::{table_banner::TableBanner, table_ui::TableUI}, utils::area::horizontal_pad_area};

// global app state.
pub struct App {
    pub input_handler: InputHandler,
    pub dataframe_state: DataFrameState,
    pub search_result_state: SearchResultState,
    pub config_state: ConfigState,
}

impl App {
    pub fn new(file_path: &str) -> Self {
        Self {
            input_handler: InputHandler::new(),
            dataframe_state: DataFrameState::new(file_path),
            search_result_state: SearchResultState::new(),
            config_state: ConfigState::new(),
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

    // the primary function for handling state conditionals
    // and updating state.
    fn step(&mut self, control: &Control) {
        Controller::perform_actions(control, self);
    }


    ///////////////
    // rendering //
    ///////////////

    fn render_frame(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let [
            _top_banner,
            _,
            main_area,
            _,
            bottom_banner,
        ] = Layout::vertical([
                Constraint::Length(1),          // top banner
                Constraint::Percentage(5),     // padding to center the app in middle.
                Constraint::Fill(1),            // main app
                Constraint::Percentage(5),     // padding
                Constraint::Length(1),          // bottom banner
            ]).areas(area);
        

        let table = TableUI::new();
        let mode_banner = ModeBanner::new();
        
        // restricting table area horizontally
        let table_area = horizontal_pad_area(main_area, [10,80,10]);
        frame.render_stateful_widget(table, table_area, self);
        frame.render_stateful_widget(mode_banner, bottom_banner, self);
    }

    fn draw<B: Backend>(&mut self, terminal: &mut Terminal<B>) {
        // let start = Instant::now();
        terminal.draw(|f| {
            self.render_frame(f);
        }).unwrap();
    }
}