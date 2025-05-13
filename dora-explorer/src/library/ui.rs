use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Stylize}, widgets::{Paragraph, StatefulWidget, Widget}};

use super::ExplorerState;

pub struct ExplorerUI {}

impl ExplorerUI {
    pub fn new() -> Self { Self{} }
}


// contain a top banner for current path
// then the left contains cwd paths.

impl ExplorerUI {
    fn render_banner(self, area: Rect, buf: &mut Buffer, state: &mut <ExplorerUI as StatefulWidget>::State) {
        let cwd = &state.cwd;
        let path = cwd
            .to_str()
            .unwrap_or("<invalid path>");

        Paragraph::new(path)
            .bg(Color::Rgb(67, 67, 113))
            .render(area, buf);
    }
}

impl StatefulWidget for ExplorerUI {
    type State = ExplorerState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [
            banner,
            main,
        ] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
        ]).areas(area);

        self.render_banner(banner, buf, state);
    }
}