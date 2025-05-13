use std::ffi::OsStr;

use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Stylize}, widgets::{Paragraph, StatefulWidget, Widget}};

use super::ExplorerState;

pub struct ExplorerUI {}

impl ExplorerUI {
    pub fn new() -> Self { Self{} }
}

const CELL_HEIGHT: u16 = 1;
const CELL_WIDTH: u16 = 30;


// contain a top banner for current path
// then the left contains cwd paths.

impl ExplorerUI {
    fn render_banner(&self, area: Rect, buf: &mut Buffer, state: &mut <ExplorerUI as StatefulWidget>::State) {
        let cwd = &state.cwd;
        let path = cwd
            .to_str()
            .unwrap_or("<invalid path>");

        Paragraph::new(path)
            .bg(Color::Rgb(67, 67, 113))
            .render(area, buf);
    }

    fn render_entries(&self, area: Rect, buf: &mut Buffer, state: &mut <ExplorerUI as StatefulWidget>::State) {
        let d_ents = &state.dents;
        let start_x = area.x;
        let start_y = area.y;

        for (idx,entry) in d_ents.iter().enumerate() {
            let curr_y = start_y + (idx as u16) * CELL_HEIGHT;
            let rect = Rect::new(start_x, curr_y, CELL_WIDTH, CELL_HEIGHT);
            let entry_str = entry
                .as_path()
                .file_name()
                .unwrap_or(OsStr::new("Invalid FileName"))
                .to_str()
                .expect("Invalid FileName");

            self.render_entry(entry_str, false, rect, buf);
        }
    }

    fn render_entry(&self, text: &str, is_dir: bool, area: Rect, buf: &mut Buffer) {
        Paragraph::new(text)
            .bg(Color::DarkGray)
            .render(area, buf)
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
        self.render_entries(main, buf, state);
    }
}