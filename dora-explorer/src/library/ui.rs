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
            let is_selected = {
                (idx as u16) == state.cursor_y
            };
            self.render_entry(entry_str, false, is_selected, rect, buf);
        }
    }

    fn render_entry(&self, text: &str, is_dir: bool, is_selected: bool, area: Rect, buf: &mut Buffer) {
        let mut para = Paragraph::new(text);

        if is_selected {
            para = para.bg(Color::Rgb(40, 40, 80))
        } else {
            para = para.bg(Color::DarkGray)
        }

        para.render(area, buf)
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