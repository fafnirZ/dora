use polars::prelude::*;
use ratatui::prelude::*;

use crate::utils::{cell::{get_cell_area, CELL_HEIGHT, CELL_WIDTH}, centered_text::render_text_centered_in_area};

#[derive(Clone)]
pub struct ColumnUI {
    values: Column, // pl.column
    // TODO: handle state such as highlighted cells etc.
    // might need to introduce another layer of objects? who knows.
}

impl ColumnUI {
    pub fn new(values: Column) -> Self {
        Self{
            values: values
        } 
    }
}

impl Widget for ColumnUI {

    fn render(self, _area: Rect, buf: &mut Buffer) {

        // render columns
        // TODO limit the number of values which are being rendered
        // based on rendering rules...
        // this is for paginating
        let start_offset = 0 as u32;
        let num_values_rendered = 50;
        let total_length_of_series = self.values.len() as u32;
        let end_offset = (start_offset + num_values_rendered)
            .min(total_length_of_series-1); // bind by total length of series

        // let binding = self
        //     .values
        //     .take_slice(&[start_offset, end_offset]).unwrap()
        //     .as_series().unwrap()
        //     .as_list();
        // let values_iter = binding.iter();

        let binding = self
            .values
            .take_slice(&[start_offset, end_offset]).unwrap();
        let binding_2 = binding
            .as_series().unwrap();
        let values_iter = binding_2.iter();

        for (idx, value) in values_iter.enumerate() {
            let x = 0; // WELL depends on what the x_offset is for this column.
            // TODO: explore making the header part of the column so its truely columnar.
            let y = CELL_HEIGHT * (idx as u16);
            let cell_area = get_cell_area(x, y);
            let val_str = value.to_string();
            render_text_centered_in_area(val_str, cell_area, buf);
        }

    }
}

