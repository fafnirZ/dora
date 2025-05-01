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
    fn render_header_borders(&self, buf: &mut Buffer, area: Rect) -> (u16, u16) {
        let block = Block::default()
            .borders(Borders::TOP | Borders::BOTTOM)
            .border_style(Style::default().fg(Color::Rgb(64, 64, 64)));
        let height = 3;
        let area = Rect::new(0, 0, area.width, height);
        block.render(area, buf);
        // y pos of header text and next line
        (height.saturating_sub(2), height)
    }
}

impl StatefulWidget for TableUI {
    type State = TableUIState;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let (y_header, y_first_record) = self.render_header_borders(buf, area);
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
}