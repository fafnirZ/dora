// infobar will be separated into 3 sections horizontally
// [search_information][buffer][message_section]

use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Stylize}, widgets::{Paragraph, StatefulWidget, Widget}};

use super::super::{colours::*, navigator::types::{DEnt, FileType}, ExplorerState};

pub struct InfoBarUI{}

impl InfoBarUI {
    pub fn new() -> Self { Self{} }


    fn render_search_info_area(self, area: Rect, buf: &mut Buffer, state: &mut <InfoBarUI as StatefulWidget>::State) {
    }
    fn render_input_buffer_area(self, area: Rect, buf: &mut Buffer, state: &mut <InfoBarUI as StatefulWidget>::State) {
    }
    fn render_output_buffer_area(self, area: Rect, buf: &mut Buffer, state: &mut <InfoBarUI as StatefulWidget>::State) {
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
        self.render_input_buffer_area(search_info_area, buf, state);
        self.render_output_buffer_area(search_info_area, buf, state);
        
    }
}