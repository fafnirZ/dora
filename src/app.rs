
use ratatui::{layout::{Constraint, Layout}, prelude::Backend, Frame, Terminal};

use crate::{controller::Controller, df::state::DataFrameState, errors::DoraResults, input::{self, Control, InputHandler}, mode_banner::{AppModeState, ModeBanner}, table::{table_banner::TableBanner, table_ui::TableUI}, utils::area::horizontal_pad_area};

// global app state.
pub struct App {
    // input_handler
    input_handler: InputHandler,
    // table_state
    // table_state: TableUIState, // thinking of deprecating this, maybe re-introduce it later as a view? who knows. theres probably many layers here.

    pub dataframe_state: DataFrameState,

    // app mode banner
    pub mode_state: AppModeState,
}

impl App {
    pub fn new(file_path: &str) -> Self {
        let mode = AppModeState::new();
        let input_handler = InputHandler::new();
        Self {
            input_handler: InputHandler::new(),
            // table_state: TableUIState::new(),
            dataframe_state: DataFrameState::new(
                file_path,
            ),
            mode_state: mode,
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
                Constraint::Percentage(35),     // padding to center the app in middle.
                Constraint::Length(30),         // main app
                Constraint::Percentage(35),     // padding
                Constraint::Length(1),          // bottom banner
            ]).areas(area);
        

        let table = TableUI::new();
        let mode_banner = ModeBanner::new();
        
        // restricting table area horizontally
        let table_area = horizontal_pad_area(main_area, [25,50,25]);
        frame.render_stateful_widget(table, table_area, self);
        frame.render_stateful_widget(mode_banner, bottom_banner, &mut self.mode_state);
    }

    fn draw<B: Backend>(&mut self, terminal: &mut Terminal<B>) {
        // let start = Instant::now();
        terminal.draw(|f| {
            self.render_frame(f);
        }).unwrap();
    }
}