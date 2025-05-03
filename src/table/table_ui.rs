use polars::frame::DataFrame;
use polars::prelude::Column;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, StatefulWidget, Widget};

use crate::app::App;
use crate::header::Header;
use crate::utils::cell::{get_cell_area, CELL_HEIGHT, CELL_WIDTH};
use crate::utils::centered_text::render_text_centered_in_area;

use super::column_ui::ColumnUI;


pub struct TableUI {
}

impl TableUI {
    pub fn new() -> Self {
        Self {}
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

        let start_x = area.x;
        let start_y = area.y;

        // rendering the block
        let block = Block::default()
            .borders(Borders::TOP | Borders::BOTTOM)
            .border_style(Style::default().fg(Color::Rgb(64, 64, 64)));
        let height = CELL_HEIGHT;
        let area = Rect::new(start_x, start_y, area.width, height);
        block.render(area, buf);


        // rendering the column values.
        let df_state = &state.dataframe_state;
        let headers = df_state.get_headers();
        for (idx, header) in headers.iter().enumerate() {
            let y = start_y;
            let x = start_x + CELL_WIDTH * (idx as u16);
            let cell_area = get_cell_area(x, y);
            let header_name = header.name.clone();
            render_text_centered_in_area(header_name, cell_area, buf);
        }

        // y pos of header text and next line
        (height.saturating_sub(2), height)
    }
}

impl StatefulWidget for TableUI {
    // only do this when u need access to more than 2 state objects
    // since you can only assign 1 object to this trait.
    type State = App; // cheat and assign to app state so we get access to everthing?

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let start_x = area.x;
        let start_y = area.y;

        let (y_header, y_first_record) = self.render_header(buf, area, state);
        let df_state = &state.dataframe_state;
        let columns = df_state.get_columns();
        for (idx, column) in columns.iter().enumerate() {
            let col_ui = ColumnUI::new(
                column.clone(),
                start_x + CELL_WIDTH * (idx as u16),
                start_y + y_first_record,
            );
            col_ui.render(area, buf);
        }
    }
    
}


// pub struct TableUIState {
//     pub dataframe: DataFrame
// }

// impl TableUIState {
  
// }