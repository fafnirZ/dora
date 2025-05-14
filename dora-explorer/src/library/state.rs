use std::{env, path::{Path, PathBuf}};

use ratatui::layout::Rect;

use super::{navigator::{local::getdents_from_path, types::DEnt}, ui::CELL_HEIGHT};


// very primitive state right now
// not optimised and not cached.
pub struct ExplorerState{
    pub cwd: PathBuf,
    pub dents: Vec<DEnt>, // directory entries

    // visual states
    pub cursor_y: u16,
    pub view_slice: [u16;2],
    available_area: [u16;2], // height, width
}

impl ExplorerState {
    pub fn new() -> Self {
        // initial path for testing purposes
        let cwd = &env::current_dir().unwrap();
        let path = cwd.as_path();
        let dents = getdents_from_path(&path).expect("Initial path is nto a directory"); 
        Self {
            cwd: path.to_path_buf(), // cwd
            dents: dents,
            cursor_y: 0,
            view_slice: [0,10], // this will be overridden very quickly
            available_area: [10, 10], // to be reset very soon.
        }
    }

    pub fn update_table_area(&mut self, main_table_area: Rect) {
        let [curr_height, curr_width] = &self.available_area;
        if !(main_table_area.height == *curr_height && main_table_area.width == *curr_width) {
            // update the table area
            self.available_area = [main_table_area.height, main_table_area.width];

            self.recalculate_view_slice();
        }
    }

    pub fn recalculate_renderable_rows(& self) -> u16 {
        let [curr_height, _] = &self.available_area;
        let max_entries = &self.dents.len();
        return (curr_height / CELL_HEIGHT)
            .min(*max_entries as u16);
    }

    pub fn recalculate_view_slice(&mut self) {
        let [start, _] = &self.view_slice;
        // let renderable_rows = self.recalculate_renderable_rows();
        self.view_slice = [*start, start+self.recalculate_renderable_rows()];
    }
}