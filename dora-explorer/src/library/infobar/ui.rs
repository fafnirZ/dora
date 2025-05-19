// infobar will be separated into 3 sections horizontally
// [search_information][buffer][message_section]

use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Stylize}, widgets::{Paragraph, StatefulWidget, Widget}};

use super::super::{colours::*, navigator::types::{DEnt, FileType}, ExplorerState};

pub struct InfoBarUI{}

impl InfoBarUI {
    pub fn new() -> Self { Self{} }
}



impl StatefulWidget for InfoBarUI {
    type State = ExplorerState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Paragraph::new("")
            .bg(MAIN_PURPLE.to_ratatui_color_rgb())
            .render(area, buf);
    }
}