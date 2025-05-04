use polars::prelude::*;
use ratatui::prelude::*;

use crate::{any_float, any_int, any_string, any_uint, app::App, utils::{cell::{get_cell_area, CELL_HEIGHT, CELL_WIDTH}, centered_text::render_text_centered_in_area}};

#[derive(Clone)]
pub struct ColumnUI {
    // values: Column, // pl.column

    column_name: String,

    // NOTE: y_offset might not be necessary
    // because we can just separate the areas in the TableUI
    // for header and column.

    x_offset: u16, // visual offset in pixels (i.e. its not the place offset but the actual pixel offset, can be changed later)
    y_offset: u16, // visual offset
    // TODO: handle state such as highlighted cells etc.
    // might need to introduce another layer of objects? who knows.
}

impl ColumnUI {
    pub fn new(column_name: String, x_offset: u16, y_offset: u16) -> Self {
        Self{
            column_name: column_name,
            x_offset: x_offset,
            y_offset: y_offset,
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

        let start_y= area.y;
        let end_y = start_y + area.height;

        // render columns
        // TODO limit the number of values which are being rendered
        // based on rendering rules...
        // this is for paginating

        // let start_offset = 0 as u32;
        // let num_values_rendered = 50;
        // let total_length_of_series = self.values.len() as u32;
        // let end_offset = (start_offset + num_values_rendered)
        //     .min(total_length_of_series-1); // bind by total length of series

        // let start_offset = 0 as i64;
        // // maybe hardcode this for now since 
        // // i dont know how to detect viewport changes
        // // this calculation probably needs to be owned somewhere else
        // // such that i can update the table banner too
        // let total_len_taken = ColumnUI::calculate_num_rows_renderable(area) as usize; 

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
            let x = self.x_offset; // WELL depends on what the x_offset is for this column.
            // TODO: explore making the header part of the column so its truely columnar.
            let y = CELL_HEIGHT * (idx as u16) + (self.y_offset as u16);

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

