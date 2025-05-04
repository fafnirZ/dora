use polars::prelude::*;
use ratatui::prelude::*;

use crate::{any_float, any_int, any_string, any_uint, app::App, cell::{get_cell_area, CELL_HEIGHT, CELL_WIDTH, HEADER_HEIGHT, LINE_NUMBER_CELL_WIDTH}, df::state::CursorFocus, utils::centered_text::{center_text_in_given_area, render_text_centered_in_area}};
// NOTE: will never add the header to column, since I dont want to be able to navigate to 
// the header? or maybe treat the header completely differently from a datastructure perspective.
// imean either way works, its just a choice I gotta deal with in implementation.

#[derive(Clone)]
pub struct LineNumberUI {
}

impl LineNumberUI {
    pub fn new() -> Self {
        Self{
        } 
    }
}


impl StatefulWidget for LineNumberUI {
    type State = App;
    
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let df_state = &state.dataframe_state;

        // NOTE: always do co-ordinate 
        // arithmetic respecting 
        // the provided area's starting positions
        // it will mean you can write code which segments the area
        // and widgets rendered within the area will automatically respect the new segmentation.
        // this is just good practice imo, otherwise its like if you decided not to use
        // flexbox in html and wanted to do position: absolute for everything...
        let start_x = area.x;
        let start_y= area.y;
        let header_offset = HEADER_HEIGHT;
        let start_y = start_y + header_offset; // need to offset the header height
        
        let [val_offset_start, val_offset_end] = df_state.get_row_view_slice();

        for (idx, line_number) in (*val_offset_start..*val_offset_end).enumerate() {
            let curr_row = line_number as usize;
            let text = curr_row.to_string();

            let cell_area = Rect::new(
                start_x,
                start_y + (idx as u16) * CELL_HEIGHT, // column
                LINE_NUMBER_CELL_WIDTH,
                CELL_HEIGHT,
            );
            let (para, text_area) = center_text_in_given_area(text, cell_area);
            para
                .fg(Color::White)
                // .bg(Color::Rgb(235, 233, 233)) // #EBE9E9
                .bg(Color::Rgb(77, 80, 97)) // #4D5061
                .render(
                    text_area,
                    buf,
                );
        }
} 
    
}

