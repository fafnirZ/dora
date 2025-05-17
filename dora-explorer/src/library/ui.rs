use std::ffi::OsStr;

use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Stylize}, widgets::{Paragraph, StatefulWidget, Widget}};

use super::{colours::{DARK_BLUE_GRAY, PALE_GREEN}, navigator::types::{DEnt, FileType}, ExplorerState};

pub struct ExplorerUI {}

impl ExplorerUI {
    pub fn new() -> Self { Self{} }
}

pub const CELL_HEIGHT: u16 = 1;
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
        
        let [vs_start, vs_end] = &state.view_slice;
        // get slice from d_ents 
        let d_ents: Vec<&DEnt> = d_ents
            .iter()
            .enumerate()
            .filter(|(idx, _)| {
                (*idx as u16) >= *vs_start
                && (*idx as u16) < *vs_end
            })
            .map(|(_idx, val)| val)
            .collect();

        
        for (idx,entry) in d_ents.iter().enumerate() {
            let curr_y = start_y + (idx as u16) * CELL_HEIGHT;

            if curr_y+CELL_HEIGHT > start_y + area.height { return; } // dont render beyong bounds

            let rect = Rect::new(start_x, curr_y, CELL_WIDTH, CELL_HEIGHT);
            let entry_str = entry
                .path
                .file_name()
                .unwrap_or("<Invalid Entry Name>");
            let is_selected = {
                (idx as u16) == state.cursor_y
            };
            self.render_entry(entry_str, &entry.ftype, is_selected, rect, buf);
        }
    }

    fn render_entry(&self, text: &str, dent_type: &FileType, is_selected: bool, area: Rect, buf: &mut Buffer) {
        let is_dir = matches!(dent_type, FileType::Dir);
        let text_to_render = {
            if is_dir {
                format!("{}", text.to_string())
            } else {
                text.to_string()
            }
        };
        let mut para = Paragraph::new(text_to_render);

        if is_selected {
            para = para.bg(DARK_BLUE_GRAY.to_ratatui_color_rgb())
        } else {
            para = para.bg(Color::DarkGray)
        }

        if is_dir {
            para = para.fg(PALE_GREEN.to_ratatui_color_rgb())
        } else {
            para = para.fg(Color::White)
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

        state.update_table_area(main.clone());

        self.render_banner(banner, buf, state);
        self.render_entries(main, buf, state);
    }
}