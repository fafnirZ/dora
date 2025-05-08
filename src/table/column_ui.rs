use polars::prelude::*;
use ratatui::prelude::*;

use crate::{any_float, any_int, any_string, any_uint, app::App, cell::{get_cell_area}, df::state::CursorFocus, mode::AppMode, utils::centered_text::center_text_in_given_area};
// NOTE: will never add the header to column, since I dont want to be able to navigate to 
// the header? or maybe treat the header completely differently from a datastructure perspective.
// imean either way works, its just a choice I gotta deal with in implementation.

#[derive(Clone)]
pub struct ColumnUI {
    column_name: String,
    column_index: u16, 
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

    fn render_cell(
        text: String,
        area: Rect,
        buf: &mut Buffer,
        is_selected: bool,
        is_search_result: bool,
    ) {

        let (para, text_area) = center_text_in_given_area(text, area);
        let mut para = para; // makes it mutable, so I can code in a certain way avoiding ownership problems.
        if is_selected {
            para = para
                .bg(Color::DarkGray);
        }
        if is_search_result {
            para = para
                .fg(Color::Red);
        } else {
            para = para
                .fg(Color::White);
        }

        para.render(
            text_area,
            buf,
        );
    }

    fn is_selected(
        curr_row: u16,
        curr_col: u16,
        state: &<ColumnUI as StatefulWidget>::State,
    ) -> bool {
        let df_state = &state.dataframe_state;

        return match df_state.get_cursor_focus() {
            CursorFocus::Column => {
                let cursor_x = *df_state.get_cursor_x() as u16;
                if curr_col == cursor_x {
                    return true;
                }
                false
            }
            CursorFocus::Row => {
                let cursor_y = *df_state.get_cursor_y() as u16;
                if curr_row == cursor_y {
                    return true;
                }
                false
            }
        }
    }

}

impl StatefulWidget for ColumnUI {
    type State = App;
    
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {

        let config_state = &state.config_state;
        let df_state = &state
            .dataframe_state;

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

        let column = df_state
            .get_column(&self.column_name);

        let [val_offset_start, val_offset_end] = df_state.get_row_view_slice();
        let length_taken = (val_offset_end-val_offset_start) as usize;
        let series = column
            .as_series()
            .unwrap()
            .slice(*val_offset_start, length_taken)
            .rechunk(); // added because of bug: https://github.com/fafnirZ/dora/issues/1
        let search_results = &state
            .search_result_state
            .result_indices;

        let search_results_found_in_row: &Vec<usize> = &search_results
            .iter()
            .map(|tuple| tuple.0)
            .collect(); 
        
        // idx is the row value relative to the slice.
        // so for search result indices which is an absolute index w.r.t.
        // the full size of the column this will be erroneous.
        for (idx, value) in series.iter().enumerate() {
            let absolute_row_index = val_offset_start + (idx as i64);

            let x = start_x + self.column_index * config_state.cell_width; // WELL depends on what the x_offset is for this column.

            let y = start_y + config_state.cell_height * (idx as u16); // respects the area bounds.

            // do not render if y is outside of area bound
            if y + config_state.cell_height > end_y {break;}

            
            let val_str = match value {
                any_int!() => value.to_string(),
                any_float!() => value.to_string(),
                any_uint!() => value.to_string(),
                any_string!() => value.to_string(),
                _ => {
                    panic!("Invalid type.")
                }
            };
            let is_selected = ColumnUI::is_selected(
                idx as u16,
                self.column_index,
                state,
            );
            let is_search_result = {
                match &state.input_handler.mode_state {
                    AppMode::Search => {
                        self.column_index == (*state.dataframe_state.get_cursor_x() as u16)
                        && (match state.dataframe_state.get_cursor_focus() { 
                            CursorFocus::Column => true, 
                            _ => false
                        })
                        && (match search_results_found_in_row.binary_search(&(absolute_row_index as usize)) {
                            Ok(_) => true,
                            _ => false,
                        })
                    }
                    _ => false
                }
            };
            let cell_area = get_cell_area(config_state, x, y);
            ColumnUI::render_cell(
               val_str,
               cell_area,
               buf,
               is_selected,
               is_search_result,
            )
        }
    }
    
}

