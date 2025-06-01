use std::ffi::OsStr;

use ratatui::{buffer::Buffer, layout::{Constraint, Direction, Layout, Rect}, style::{Color, Stylize}, widgets::{Paragraph, StatefulWidget, Widget}};

use super::{colours::*, internal::{node, node_path::NodePath}, ExplorerState};

pub struct ExplorerUI {}

impl ExplorerUI {
    pub fn new() -> Self { Self{} }
}

pub const CELL_HEIGHT: u16 = 1;
// const CELL_WIDTH: u16 = 30;


// contain a top banner for current path
// then the left contains cwd paths.

impl ExplorerUI {
}

impl ExplorerUI {
    fn render_main(&self, area: Rect, buf: &mut Buffer, state: &mut <ExplorerUI as StatefulWidget>::State) {
        let data = {
            let mut result = String::new();
            for (str, _ ) in &state.root_node_structure {
                result += &str;
            }
            result
        };
        let contents: Vec<&str> = data.split("\n").collect();

        // dynamically break up the areas available into lines.
        let available_height = area.height;
        let constraint_vec: Vec<Constraint> = (0..available_height)
            .map(|_| Constraint::Length(1))
            .collect();
        let lines = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraint_vec)
            .split(area); 

        // rendering line
        for (idx, line) in lines.iter().enumerate() {
            if idx > contents.len()-1 {
                break;
            }
            let cur_content = contents[idx];

            let mut para = Paragraph::new(cur_content);
            
            if (idx as u16) == state.cursor_y {
                para = para.bg(
            DARK_TEAL.to_ratatui_color_rgb()
                );
            }

            para.render(*line, buf); 

        }
    }

    fn render_bottom_banner(&self, area: Rect, buf: &mut Buffer, state: &mut <ExplorerUI as StatefulWidget>::State) {
        let cursor_y = &state.cursor_y; // todo need absolute cursor in future
        let node_paths: Vec<NodePath> = state.root_node_structure
            .clone()
            .into_iter()
            .map(|(_, node_path)| node_path)
            .collect();

        let node_path = &node_paths[*(cursor_y) as usize];

        Paragraph::new(node_path.to_string())
            .bg(DARK_TEAL.to_ratatui_color_rgb())
            .render(area, buf);
    }
}


impl StatefulWidget for ExplorerUI {
    type State = ExplorerState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [
            top_banner,
            main,
            bottom_banner,
        ] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]).areas(area);
        
        state.update_table_area(main);
        self.render_main(main, buf, state);
        self.render_bottom_banner(bottom_banner, buf, state);
    }
}