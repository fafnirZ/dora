use polars::prelude::*;
use ratatui::{
    prelude::*,
    widgets::{Paragraph, Wrap},
};

use crate::{
    any_datetime, any_float, any_int, any_string, any_uint, app::App, df::state::CursorFocus, mode::AppMode, utils::cell::get_cell_area
};
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
        Self {
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
        is_word_wrap: bool,
        is_selected: bool,
        is_search_result: bool,
    ) {
        let mut style = Style::new();
        if is_selected {
            style = style.bg(Color::DarkGray);
        }
        if is_search_result {
            style = style.fg(Color::Red);
        } else {
            style = style.fg(Color::White);
        }

        // Fill(0) allows collapsing to size 0 when needed?
        let [cell_top_padding, text_area, cell_bottom_padding] =
            Layout::vertical([Constraint::Fill(0), Constraint::Min(1), Constraint::Fill(0)])
                .areas(area);

        Paragraph::new("")
            .style(style)
            .render(cell_top_padding, buf);
        Paragraph::new("")
            .style(style)
            .render(cell_bottom_padding, buf);

        // main cell contents;
        let mut cell_contents = Paragraph::new(text)
            .style(style)
            .alignment(Alignment::Center);

        // determine whether to word wrap or not
        if is_word_wrap {
            cell_contents = cell_contents.wrap(Wrap { trim: false });
        }
        // render
        cell_contents.render(text_area, buf);
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
        };
    }
    fn is_search_result(
        column_index: u16,
        search_results_found_in_row: &Vec<usize>,
        absolute_row_index: i64,
        state: &<ColumnUI as StatefulWidget>::State,
    ) -> bool {
        match state.input_handler.mode_state {
            AppMode::Search => {
                column_index == (*state.dataframe_state.get_cursor_x() as u16)
                    && (match state.dataframe_state.get_cursor_focus() {
                        CursorFocus::Column => true,
                        _ => false,
                    })
                    && (match search_results_found_in_row
                        .binary_search(&(absolute_row_index as usize))
                    {
                        Ok(_) => true,
                        _ => false,
                    })
            }
            _ => false,
        }
    }
    fn is_word_wrap(state: &<ColumnUI as StatefulWidget>::State) -> bool {
        state.config_state.word_wrap
    }
}

impl StatefulWidget for ColumnUI {
    type State = App;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let config_state = &state.config_state;
        let df_state = &state.dataframe_state;

        // NOTE: always do co-ordinate
        // arithmetic respecting
        // the provided area's starting positions
        // it will mean you can write code which segments the area
        // and widgets rendered within the area will automatically respect the new segmentation.
        // this is just good practice imo, otherwise its like if you decided not to use
        // flexbox in html and wanted to do position: absolute for everything...
        let start_x = area.x;
        let start_y = area.y;
        let end_y = start_y + area.height;

        let column = df_state.get_column(&self.column_name);

        let [val_offset_start, val_offset_end] = df_state.get_row_view_slice();
        let length_taken = (val_offset_end - val_offset_start) as usize;
        let series = column
            .as_series()
            .unwrap()
            .slice(*val_offset_start, length_taken)
            .rechunk(); // added because of bug: https://github.com/fafnirZ/dora/issues/1
        let search_results = &state.search_result_state.result_indices;

        let search_results_found_in_row: &Vec<usize> =
            &search_results.iter().map(|tuple| tuple.0).collect();

        let is_word_wrap = ColumnUI::is_word_wrap(state);

        // idx is the row value relative to the slice.
        // so for search result indices which is an absolute index w.r.t.
        // the full size of the column this will be erroneous.
        for (idx, value) in series.iter().enumerate() {
            let absolute_row_index = val_offset_start + (idx as i64);

            let x = start_x + self.column_index * config_state.cell_width; // WELL depends on what the x_offset is for this column.

            let y = start_y + config_state.cell_height * (idx as u16); // respects the area bounds.

            // do not render if y is outside of area bound
            if y + config_state.cell_height > end_y {
                break;
            }

            let val_str = match value {
                any_int!() => value.to_string(),
                any_float!() => value.to_string(),
                any_uint!() => value.to_string(),
                any_string!() => value.to_string(),
                any_datetime!() => value.to_string(),
                AnyValue::Null => "None".to_string(),
                AnyValue::Boolean(value) => value.to_string(),

                _ => {
                    panic!("Invalid type.")
                }
            };
            let is_selected = ColumnUI::is_selected(idx as u16, self.column_index, state);
            let is_search_result = ColumnUI::is_search_result(
                self.column_index,
                search_results_found_in_row,
                absolute_row_index,
                state,
            );
            let cell_area = get_cell_area(config_state, x, y);
            ColumnUI::render_cell(
                val_str,
                cell_area,
                buf,
                is_word_wrap,
                is_selected,
                is_search_result,
            )
        }
    }
}
