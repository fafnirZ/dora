use polars::prelude::*;
use ratatui::prelude::*;

use crate::{any_float, any_int, any_string, any_uint, app::App, cell::{get_cell_area, CELL_HEIGHT, CELL_WIDTH}, df::state::CursorFocus, utils::centered_text::{center_text_in_given_area, render_text_centered_in_area}};
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
        let end_y = start_y + area.height;
        
        let [val_offset_start, val_offset_end] = df_state.get_row_view_slice();

        for i in *val_offset_start..*val_offset_end {
            let curr_row = i as usize;
            let text = curr_row.to_string();
    
            let (para, text_area) = center_text_in_given_area(text, area);
            para.render(
                text_area,
                buf,
            );
        }
} 
    
}

