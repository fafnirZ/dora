use polars::frame::DataFrame;
use polars::prelude::Column as PlColumn;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph, StatefulWidget, Widget};

use crate::header::Header;
use crate::column_ui::ColumnUI;
use crate::utils::cell::{get_cell_area, CELL_HEIGHT, CELL_WIDTH};
use crate::utils::centered_text::render_text_centered_in_area;


pub struct TableUI {
    header: Vec<Header>,
    columns: Vec<ColumnUI>,
}

impl TableUI {
    pub fn new(header: Vec<Header>, columns: Vec<ColumnUI>) -> Self {
        Self {
            header,
            columns,
        }
    }
}

// priv
impl TableUI {
    fn render_header(
        &self, 
        buf: &mut Buffer, 
        area: Rect, 
        state: &mut <TableUI as StatefulWidget>::State
    ) -> (u16, u16) {
        let block = Block::default()
            .borders(Borders::TOP | Borders::BOTTOM)
            .border_style(Style::default().fg(Color::Rgb(64, 64, 64)));
        let height = CELL_HEIGHT;
        let area = Rect::new(0, 0, area.width, height);
        block.render(area, buf);
        
        let headers = state.get_headers();
        for (idx, header) in headers.iter().enumerate() {
            let y = 0;
            let x = CELL_WIDTH * (idx as u16);
            let cell_area = get_cell_area(x, y);
            let header_name = header.name.clone();
            render_text_centered_in_area(header_name, cell_area, buf);
        }

        // y pos of header text and next line
        (height.saturating_sub(2), height)
    }
}

impl StatefulWidget for TableUI {
    type State = TableUIState;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let (y_header, y_first_record) = self.render_header(buf, area, state);
        let columns = self.columns;
        for column in columns.iter() {
            column.clone().render(area, buf);
        }
    }
    
}


pub struct TableUIState {
    pub dataframe: DataFrame
}

impl TableUIState {
    pub fn new() -> Self {
        // boilerplate df for now
        let s0 = PlColumn::new("days".into(), [0, 1, 2].as_ref());
        let s1 = PlColumn::new("temp".into(), [22.1, 19.9, 7.].as_ref());
        let df = DataFrame::new(vec![s0, s1]).unwrap();
        Self {
            dataframe: df,
        }
    }

    pub fn get_headers(&self) -> Vec<Header> {
        let df = &self.dataframe;
        
        let df_schema = df.schema();
        let mut headers: Vec<Header> = vec![];
        for (col_name, _dt) in df_schema.iter() {
            headers.push(
                Header{name: col_name.to_string()}
            );
        }
        headers
    }
    pub fn get_columns(&self) -> Vec<ColumnUI> {
        let df = &self.dataframe;
        // get columns
        let mut columns = vec![];
        for col_name in self.get_headers() {
            let col = df.column(&col_name.name).unwrap();
            // let dt = series.dtype();
            columns.push(
                ColumnUI::new(col.clone()), // copy for now 
            )
        }
        columns
    }
}