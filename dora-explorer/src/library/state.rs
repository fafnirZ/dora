use std::{env, path::{Path, PathBuf}};

use google_cloud_storage::client::Client;
use ratatui::layout::Rect;

use super::{navigator::{ gcs::GCSNavigator, local::getdents_from_path, traits::AnyPath, types::DEnt}, ui::CELL_HEIGHT};


// very primitive state right now
// not optimised and not cached.
pub struct ExplorerState{
    pub cwd: AnyPath,
    pub dents: Vec<DEnt>, // directory entries
    pub cloud_client: Option<Client>,

    // visual states
    pub cursor_y: u16,
    pub view_slice: [u16;2],
    available_area: [u16;2], // height, width
}

impl ExplorerState {
    pub fn new(file_path: Option<String>) -> Self {
        
        if let Some(path) = file_path {
            if path.starts_with("gs://") {
                let path_shadow = path.clone();
                let gs_path = AnyPath::GSPath(path);
                let cloud_client = GCSNavigator::get_client().unwrap();
                let dents = GCSNavigator::getdents_from_path(
                    &cloud_client,
                    &path_shadow,
                ).unwrap();
                return Self {
                    cwd: gs_path, // cwd
                    dents: dents,
                    cloud_client: Some(cloud_client),
                    cursor_y: 0,
                    view_slice: [0,10], // this will be overridden very quickly
                    available_area: [10, 10], // to be reset very soon.
                }
            } else {
                //
                panic!("TODO");
            }
        } else {
            // use cwd
            // initial path for testing purposes
            // no remote path unless explicitly arg passed in begins with gs://
            let local_cwd = env::current_dir().unwrap();
            let cwd = AnyPath::LocalPath(local_cwd.clone());
            let dents = getdents_from_path(&local_cwd).expect("Initial path is nto a directory"); 
            return Self {
                cwd: cwd, // cwd
                dents: dents,
                cloud_client: None,
                cursor_y: 0,
                view_slice: [0,10], // this will be overridden very quickly
                available_area: [10, 10], // to be reset very soon.
            }
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