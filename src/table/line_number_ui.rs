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

        let df_state = &state
            .dataframe_state;
        let column = df_state
            .get_column(&self.column_name);

        let [val_offset_start, val_offset_end] = df_state.get_row_view_slice();
        let length_taken = (val_offset_end-val_offset_start) as usize;
        let series = column
            .as_series()
            .unwrap()
            .slice(*val_offset_start, length_taken)
            .rechunk(); // added because of bug: https://github.com/fafnirZ/dora/issues/1
         
        for (idx, value) in series.iter().enumerate() {
            let x = start_x + self.column_index * CELL_WIDTH; // WELL depends on what the x_offset is for this column.

            let y = start_y + CELL_HEIGHT * (idx as u16); // respects the area bounds.

            // do not render if y is outside of area bound
            if y + CELL_HEIGHT > end_y {break;}

            
            let val_str = match value {
                any_int!() => value.to_string(),
                any_float!() => value.to_string(),
                any_uint!() => value.to_string(),
                any_string!() => value.to_string(),
                _ => {
                    panic!("Invalid type.")
                }
            };
            let cell_area = get_cell_area(x, y);
            let is_selected = LineNumberUI::is_selected(
                idx as u16,
                self.column_index,
                state,
            );
            LineNumberUI::render_cell(
               val_str,
               cell_area,
               buf,
               is_selected,
            )
        }
    }
    
}

