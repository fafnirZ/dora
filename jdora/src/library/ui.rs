use std::ffi::OsStr;

use ratatui::{buffer::Buffer, layout::{Constraint, Direction, Layout, Rect}, style::{Color, Stylize}, widgets::{Paragraph, StatefulWidget, Widget}};

use super::{colours::*, ExplorerState};

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
        let data = state.node_state.pprint();
        let contents: Vec<&str> = "\n".split(&data).collect();

        let available_height = area.height;
        let constraint_vec: Vec<Constraint> = (0..available_height)
            .map(|_| Constraint::Length(1))
            .collect();
        let lines = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraint_vec)
            .split(area); 

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
        
        self.render_main(main, buf, state);
    }
}