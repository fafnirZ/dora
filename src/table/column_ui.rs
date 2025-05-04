use polars::prelude::*;
use ratatui::prelude::*;

use crate::{any_float, any_int, any_string, any_uint, app::App, utils::{cell::{get_cell_area, CELL_HEIGHT, CELL_WIDTH}, centered_text::render_text_centered_in_area}};

#[derive(Clone)]
pub struct ColumnUI {
    // values: Column, // pl.column

    column_name: String,
    column_index: u16, 
    // TODO: handle state such as highlighted cells etc.
    // might need to introduce another layer of objects? who knows.
}

impl ColumnUI {
    pub fn new(column_name: String, column_index: u16) -> Self {
        Self{
            column_name: column_name,
            column_index: column_index,
        } 
    }
}

impl ColumnUI {
    pub fn calculate_num_rows_renderable(
        area: Rect,
    ) -> u16 {
        return ((area.height / CELL_HEIGHT) as f64).floor() as u16;
    }
}

impl StatefulWidget for ColumnUI {
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

        let [val_offset_start, val_offset_end] = df_state.get_view_slice();
        let length_taken = (val_offset_end-val_offset_start) as usize;
        let series = column
            .as_series()
            .unwrap()
            .slice(*val_offset_start, length_taken)
            .rechunk(); // added because of bug: https://github.com/fafnirZ/dora/issues/1
         
        for (idx, value) in series.iter().enumerate() {
            let x = start_x + self.column_index * CELL_WIDTH; // WELL depends on what the x_offset is for this column.
            // TODO: explore making the header part of the column so its truely columnar.
            let y = start_y + CELL_HEIGHT * (idx as u16); // respects the area bounds.

            // do not render if y is outside of area bound
            if y + CELL_HEIGHT > end_y {break;}

            let cell_area = get_cell_area(x, y);

            let val_str = match value {
                any_int!() => value.to_string(),
                any_float!() => value.to_string(),
                any_uint!() => value.to_string(),
                any_string!() => value.to_string(),
                _ => {
                    panic!("Invalid type.")
                }
            };
            render_text_centered_in_area(val_str, cell_area, buf);

        }
    }
    
}

