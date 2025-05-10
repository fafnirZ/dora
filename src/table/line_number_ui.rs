use polars::prelude::*;
use ratatui::prelude::*;

use crate::{app::App, utils::centered_text::{render_text_centered_text_with_style}};
// NOTE: will never add the header to column, since I dont want to be able to navigate to
// the header? or maybe treat the header completely differently from a datastructure perspective.
// imean either way works, its just a choice I gotta deal with in implementation.

#[derive(Clone)]
pub struct LineNumberUI {}

impl LineNumberUI {
    pub fn new() -> Self {
        Self {}
    }
}

impl StatefulWidget for LineNumberUI {
    type State = App;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let df_state = &state.dataframe_state;
        let config_state = &state.config_state;

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
        let header_offset = config_state.header_height;
        let start_y = start_y + header_offset; // need to offset the header height

        let [val_offset_start, val_offset_end] = df_state.get_row_view_slice();

        for (idx, line_number) in (*val_offset_start..*val_offset_end).enumerate() {
            let curr_row = line_number as usize;
            let text = curr_row.to_string();

            let y = start_y + (idx as u16) * config_state.cell_height; // column
            // do not render if y is outside of area bound
            if y > end_y {
                break;
            }
            let cell_area = Rect::new(
                start_x,
                y,
                config_state.line_number_cell_width,
                config_state.cell_height,
            );

            let style = Style::new()
                .fg(Color::White)
                .bg(Color::Rgb(77, 80, 97));

            render_text_centered_text_with_style(
                text,
                cell_area,
                style,
                buf,
            )
        }
    }
}
