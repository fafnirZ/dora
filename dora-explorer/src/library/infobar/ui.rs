// infobar will be separated into 3 sections horizontally
// [search_information][buffer][message_section]

use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Stylize}, widgets::{Paragraph, StatefulWidget, Widget}};

use crate::library::input::InputBuffer;

use super::super::{colours::*, navigator::types::{DEnt, FileType}, ExplorerState};

pub struct InfoBarUI{}

impl InfoBarUI {
    pub fn new() -> Self { Self{} }


    fn render_search_info_area(&self, area: Rect, buf: &mut Buffer, state: &mut <InfoBarUI as StatefulWidget>::State) {
        let absolute_cursor_pos = &state.cursor_y + &state.view_slice[0];
        let curr_dent_val = (absolute_cursor_pos+1).min(0); // print 0 if non exists

        // assuming no pagination RIGHT NOW.
        // cbb dealing with pagination yet.
        let total_dent_values = &state.dents.len();

        let fmtted_str = format!("{}/{} Entries", curr_dent_val, total_dent_values);

        Paragraph::new(fmtted_str)
            .render(area, buf);
    }
    fn render_input_buffer_area(&self, area: Rect, buf: &mut Buffer, state: &mut <InfoBarUI as StatefulWidget>::State) {
        let input_buffer_str = {
            match &state.input_handler.input_buffer {
                InputBuffer::Active(buffer) => {   
                    format!("Filter:{}", buffer.value())
                },
                InputBuffer::Inactive => "".to_string(),
            }
        };
        

        Paragraph::new(input_buffer_str)
            .render(area, buf);
    }
    fn render_output_buffer_area(&self, area: Rect, buf: &mut Buffer, state: &mut <InfoBarUI as StatefulWidget>::State) {
    }

}



impl StatefulWidget for InfoBarUI {
    type State = ExplorerState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // paint entire area a colour
        Paragraph::new("")
            .bg(MAIN_PURPLE.to_ratatui_color_rgb())
            .render(area, buf);

        let [
            search_info_area,
            input_buffer_area,
            output_buffer_area,
        ] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ]).areas(area);

        self.render_search_info_area(search_info_area, buf, state);
        self.render_input_buffer_area(input_buffer_area, buf, state);
        self.render_output_buffer_area(output_buffer_area, buf, state);
        
    }
}