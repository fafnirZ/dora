// this is the stateful widget in charge of
// rendering a full page where the user
// can pick the tabs they wish to select
// for the excel reader.

use ratatui::{
    buffer::Buffer, layout::{Alignment, Constraint, Layout, Rect}, style::{Color, Stylize}, widgets::{Block, Borders, Paragraph, StatefulWidget, Widget}
};

pub struct ExcelSheetSelectorWidgetState {
    pub sheet_names: Option<Vec<String>>,
    pub cursor: u16,
}

impl ExcelSheetSelectorWidgetState {
    pub fn new() -> Self {
        Self {
            sheet_names: None,
            cursor: 0,
        }
    }
}

// const ELEMENT_WIDTH: u16 = 10;
const ELEMENT_HEIGHT: u16 = 1;

pub struct ExcelSheetSelectorPage {}

impl ExcelSheetSelectorPage {
    pub fn new() -> Self {
        Self {}
    }

    fn render_element(&self, sheet_name: &str, is_selected: bool, area: Rect, buf: &mut Buffer) {
        // v align
        let [
            _,
            main,
            _,
        ] = Layout::vertical([
            Constraint::Fill(0),
            Constraint::Min(1),
            Constraint::Fill(0),
        ]).areas(area);


        let mut para = Paragraph::new(sheet_name)
            .alignment(Alignment::Center);
            
        if is_selected {
            para = para.bg(Color::DarkGray);
        }

        para.render(main, buf);
    }

    fn render_widget(
        &self,
        area: Rect,
        buf: &mut Buffer,
        state: &mut <ExcelSheetSelectorPage as StatefulWidget>::State,
    ) {
        let start_x = area.x;
        let start_y = area.y;
        let sheet_names = state
            .sheet_names
            .as_ref()
            .expect("excel file doesnt have sheet names? is this possible?");

        for (idx, sheet_name) in sheet_names.iter().enumerate() {
            let curr_y = start_y + (idx as u16) * ELEMENT_HEIGHT;
            // fills width to the size of the element.
            let area = Rect::new(start_x, curr_y, area.width, ELEMENT_HEIGHT);
            // render a boarder around the block.
            Block::new()
                .borders(Borders::ALL)
                .render(area, buf);

            // highlights if cursor is on it.
            let is_selected = {
                (idx as u16) == state.cursor
            };
            self.render_element(sheet_name, is_selected, area, buf);
        }
    }
}

impl StatefulWidget for ExcelSheetSelectorPage {
    type State = ExcelSheetSelectorWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // padding vertical
        let [_, main, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Percentage(50),
            Constraint::Fill(1),
        ])
        .areas(area);

        // padding horizontal
        let [_, main, _] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Percentage(10),
            Constraint::Fill(1),
        ])
        .areas(main);

        self.render_widget(main, buf, state);
    }
}
