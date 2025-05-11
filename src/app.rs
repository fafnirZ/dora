use polars::prelude::file;
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
    io::{get_cursor_from_any_path, ExcelReader, ExcelSheetSelectorWidgetState},
    mode_banner::ModeBanner,
    page::{self, PageState},
    search::state::SearchResultState,
    table::table_ui::TableUI,
    utils::area::horizontal_pad_area,
};

// global app state.
pub struct App {
    // global states (regardless of page)
    pub input_handler: InputHandler,
    pub config_state: ConfigState,
    pub page_state: PageState,

    // normal page state
    pub dataframe_state: DataFrameState,
    pub search_result_state: SearchResultState,

    // excel sheet selector page state
    pub sheet_selector_state: ExcelSheetSelectorWidgetState,
}

impl App {
    pub fn new(file_path: &str) -> Self {
        // determining page state.
        let page_state = {
            if PageState::determine_is_multisheet_selection_page(file_path) {
                PageState::MultiSheetSelectionPage
            } else {
                PageState::TablePage
            }
        };

        // dataframe state
        let mut dataframe_state = DataFrameState::new(file_path);
        match page_state {
            PageState::MultiSheetSelectionPage => {} // dataframe wont be evaluated until collect is called later
            _ => {
                dataframe_state.collect(); // will evaluate the dataframe upfront
            }
        }

        // sheet_selector_state 
        let sheet_selector_state = match page_state {
            PageState::MultiSheetSelectionPage => {
                let cursor = get_cursor_from_any_path(file_path)
                    .unwrap();
                let worksheet_names = ExcelReader::new(cursor)
                    .get_worksheet_names()
                    .unwrap();
                ExcelSheetSelectorWidgetState{
                    sheet_names: Some(worksheet_names),
                    cursor: 0,
                }
            } // dataframe wont be evaluated until collect is called later
            _ => {
                ExcelSheetSelectorWidgetState::new()
            }
        };

        Self {
            input_handler: InputHandler::new(),
            dataframe_state: dataframe_state,
            search_result_state: SearchResultState::new(),
            config_state: ConfigState::new(),
            page_state: page_state,
            sheet_selector_state: sheet_selector_state,
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
        match self.page_state {
            PageState::TablePage => {
                self.render_table(frame);
            }
            PageState::MultiSheetSelectionPage => {}
        }
    }
    fn render_table(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let [top_banner, _, main_area, _, bottom_banner] = Layout::vertical([
            Constraint::Length(1),     // top banner
            Constraint::Percentage(5), // padding to center the app in middle.
            Constraint::Fill(1),       // main app
            Constraint::Percentage(5), // padding
            Constraint::Length(1),     // bottom banner
        ])
        .areas(area);

        let table = TableUI::new();
        let mode_banner = ModeBanner::new();

        // restricting table area horizontally
        let table_area = horizontal_pad_area(main_area, [10, 80, 10]);
        frame.render_stateful_widget(table, table_area, self);
        frame.render_stateful_widget(mode_banner, bottom_banner, self);

        // top banner
        let top_banner_widget =
            Paragraph::new("Dora (A Dataframe exploration tool)").bg(Color::Rgb(67, 67, 113));
        frame.render_widget(top_banner_widget, top_banner);
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
