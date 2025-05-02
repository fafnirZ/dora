use polars::frame::DataFrame;
use polars::prelude::Column as PlColumn;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, StatefulWidget, Widget};

use crate::header::Header;
use crate::column::Column;


pub struct TableUI {
    header: Vec<Header>,
    columns: Vec<Column>,
}

impl TableUI {
    pub fn new(header: Vec<Header>, columns: Vec<Column>) -> Self {
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
        let height = 3;
        let area = Rect::new(0, 0, area.width, height);
        block.render(area, buf);
        
        // get headers
        // hacking it for now
        let headers = state.get_headers();
        let header_str = headers.iter()
            .map(|h| h.name.to_string())
            .reduce(|a, b| a + &b)
            .unwrap();
        buf.set_stringn(0,0, header_str, area.width as usize, Style::default());

        // y pos of header text and next line
        (height.saturating_sub(2), height)
    }
}

impl StatefulWidget for TableUI {
    type State = TableUIState;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let (y_header, y_first_record) = self.render_header(buf, area, state);
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
    pub fn get_columns(&self) -> Vec<Column> {
        let df = &self.dataframe;
        // get columns
        let mut columns = vec![];
        for col_name in self.get_headers() {
            let col = df.column(&col_name.name).unwrap();
            // let dt = series.dtype();
            columns.push(
                Column::new(col.clone()),
            )
        }
        columns
    }
}