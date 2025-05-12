// this is the stateful widget in charge of
// rendering a full page where the user
// can pick the tabs they wish to select
// for the excel reader.

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget, Wrap},
};

///////////
// state //
///////////
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

//////////////
// renderer //
//////////////
// const ELEMENT_WIDTH: u16 = 10;
const ELEMENT_HEIGHT: u16 = 1;

pub struct ExcelSheetSelectorPage {}

impl ExcelSheetSelectorPage {
    pub fn new() -> Self {
        Self {}
    }

    fn render_element(&self, sheet_name: &str, is_selected: bool, area: Rect, buf: &mut Buffer) {
        // v align
        let [_, main, _] =
            Layout::vertical([Constraint::Fill(1), Constraint::Min(1), Constraint::Fill(1)])
                .areas(area);

        let mut para = Paragraph::new(sheet_name).alignment(Alignment::Center);

        if is_selected {
            para = para.fg(Color::White).bg(Color::DarkGray);
        } else {
            para = para.fg(Color::Black).bg(Color::Gray);
        }

        para.render(main, buf);
    }

    fn render_widget(
        &self,
        area: Rect,
        buf: &mut Buffer,
        state: &mut <ExcelSheetSelectorPage as StatefulWidget>::State,
    ) {
        // split area up vertically
        let [heading, main, footer] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .areas(area);
        // heading
        Paragraph::new("Select the sheet you wish to view")
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center)
            .render(heading, buf);

        // footer
        Paragraph::new("Press <Enter> to select your sheet")
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center)
            .render(footer, buf);

        // main section
        let start_x = main.x;
        let start_y = main.y;
        let sheet_names = state
            .sheet_names
            .as_ref()
            .expect("excel file doesnt have sheet names? is this possible?");

        for (idx, sheet_name) in sheet_names.iter().enumerate() {
            let curr_y = start_y + ((idx as u16) * ELEMENT_HEIGHT);
            // fills width to the size of the element.
            let main_area = Rect::new(start_x, curr_y, area.width, ELEMENT_HEIGHT + 2);

            // highlights if cursor is on it.
            let is_selected = { (idx as u16) == state.cursor };
            self.render_element(sheet_name, is_selected, main_area, buf);
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
            Constraint::Percentage(20),
            Constraint::Fill(1),
        ])
        .areas(main);

        self.render_widget(main, buf, state);
    }
}
