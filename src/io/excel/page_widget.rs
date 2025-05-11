// this is the stateful widget in charge of 
// rendering a full page where the user
// can pick the tabs they wish to select 
// for the excel reader.

use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, widgets::StatefulWidget};

pub struct ExcelSheetSelectorWidgetState {
    sheet_names: Vec<String>,
    cursor: u16,
}

pub struct ExcelSheetSelectorPage {}

impl ExcelSheetSelectorPage {
    pub fn new() -> Self { Self {} }

    fn render_widget(&self, area: Rect, buf: &mut Buffer, state: &mut <ExcelSheetSelectorPage as StatefulWidget>::State) {

    }
}


impl StatefulWidget for ExcelSheetSelectorPage {
    type State = ExcelSheetSelectorWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // padding vertical
        let [
            _,
            main,
            _,
        ] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Percentage(50),
            Constraint::Fill(1),
        ]).areas(area);

        // padding horizontal
        let [
            _,
            main,
            _,
        ] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Percentage(50),
            Constraint::Fill(1),
        ]).areas(main);

        self.render_widget(main, buf, state);

    }


}