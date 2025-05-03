use polars::prelude::*;
use ratatui::prelude::*;

use crate::utils::{cell::{get_cell_area, CELL_HEIGHT, CELL_WIDTH}, centered_text::render_text_centered_in_area};

#[derive(Clone)]
pub struct ColumnUI {
    values: Column, // pl.column
    x_offset: u16, // visual offset in pixels (i.e. its not the place offset but the actual pixel offset, can be changed later)
    y_offset: u16, // visual offset
    // TODO: handle state such as highlighted cells etc.
    // might need to introduce another layer of objects? who knows.
}

impl ColumnUI {
    pub fn new(values: Column, x_offset: u16, y_offset: u16) -> Self {
        Self{
            values: values,
            x_offset: x_offset,
            y_offset: y_offset,
        } 
    }
}

impl Widget for ColumnUI {

    fn render(self, area: Rect, buf: &mut Buffer) {

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

        let start_offset = 0 as i64;
        let total_len_taken = 10 as usize;

        let binding = self
            .values
            .slice(start_offset, total_len_taken);
        let binding_2 = binding
            .as_series().unwrap();
        let values_iter = binding_2.iter();

        for (idx, value) in values_iter.enumerate() {
            
            let x = self.x_offset; // WELL depends on what the x_offset is for this column.
            // TODO: explore making the header part of the column so its truely columnar.
            let y = CELL_HEIGHT * (idx as u16) + (self.y_offset as u16);

            // do not render if y is outside of area bound
            if y + CELL_HEIGHT > end_y {break;}

            let cell_area = get_cell_area(x, y);
            let val_str = value.to_string();
            render_text_centered_in_area(val_str, cell_area, buf);
        }

    }
}

