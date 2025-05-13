use ratatui::{
    Frame, Terminal,
    layout::{Constraint, Layout},
    prelude::Backend,
    style::{Color, Stylize},
    widgets::Paragraph,
};

use crate::{
    config::ConfigState,
    controller::Controller,
    df::state::DataFrameState,
    errors::DoraResults,
    input::{Control, InputHandler},
    io::{
        ExcelReader, ExcelSheetSelectorPage, ExcelSheetSelectorWidgetState,
        get_cursor_from_any_path,
    },
    mode::AppMode,
    mode_banner::ModeBanner,
    page::{PageState},
    search::state::SearchResultState,
    table::table_ui::TableUI,
    utils::area::horizontal_pad_area,
};

// global app state.
pub struct App {
    // global states (regardless of page)
    pub input_handler: InputHandler,
}

impl App {
    pub fn new() -> Self {
        Self {
            input_handler: InputHandler::new(),
        }
    }

    pub fn main_loop<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> DoraResults<Option<String>> {
        loop {
            let control = self.input_handler.next();
            if matches!(control, Control::Quit) {
                return Ok(None);
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
        todo()
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
