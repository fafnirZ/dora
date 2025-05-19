use std::{any::Any, env, path::{Path, PathBuf}};

use google_cloud_storage::client::Client;
use ratatui::layout::Rect;

use super::{input::InputHandler, mode::Mode, navigator::{ gcs::GCSNavigator, local::{filter_dot_dent, getdents_from_path}, traits::{AnyNavigator, AnyPath, Navigator}, types::DEnt}, ui::CELL_HEIGHT};


// very primitive state right now
// not optimised and not cached.
pub struct ExplorerState{

    // navigator states
    pub cwd: AnyPath,
    pub dents: Vec<DEnt>, // directory entries
    pub dents_filterview: Option<Vec<DEnt>>, // shadows dents..it is used when a filter is being applied, it will clone the original dents value in case we want to revert, which we very well may want to.
    pub cloud_client: Option<Client>,
    pub navigator: AnyNavigator,

    // configs
    pub show_dotfiles: bool,

    // visual states
    pub cursor_y: u16,
    pub view_slice: [u16;2],
    available_area: [u16;2], // height, width

    // input handler
    pub input_handler: InputHandler,
    pub mode: Mode,

    // trap signals
    // when these gets flagged, it will exit.
    pub sig_user_input_exit: bool,
    pub sig_file_selected_exit: bool,
}

impl ExplorerState {
    pub fn new(file_path: Option<String>) -> Self {
        
        if let Some(path_str) = file_path {
            if path_str.starts_with("gs://") {
                return ExplorerState::handle_init_gcs_path(path_str);
            } else {
                let path = Path::new(&path_str); // NOTE: ~ and $HOME alias works, as well as any env variables which resolves into paths :)
                return ExplorerState::handle_init_local_path(path);
            }
        } else {
            // use cwd
            // initial path for testing purposes
            // no remote path unless explicitly arg passed in begins with gs://
            let local_cwd = env::current_dir().unwrap();
            return ExplorerState::handle_init_local_path(&local_cwd);           
        }
    }

    fn handle_init_gcs_path(path: String) -> Self {
        let path_shadow = path.clone();
        let gs_path = AnyPath::GSPath(
            AnyPath::ensure_trailing_slash(path)
        );
        let cloud_client = GCSNavigator::get_client().unwrap();
        let dents = GCSNavigator::getdents_from_path(
            &cloud_client,
            &path_shadow,
        ).unwrap();
        return Self {
            cwd: gs_path, // cwd
            dents: dents,
            dents_filterview: None,
            cloud_client: Some(cloud_client),
            navigator: AnyNavigator::GCSNavigator,
            show_dotfiles: true,
            cursor_y: 0,
            view_slice: [0,10], // this will be overridden very quickly
            available_area: [10, 10], // to be reset very soon.
            sig_user_input_exit: false,
            sig_file_selected_exit: false,
            input_handler: InputHandler::new(),
            mode: Mode::Normal,
        }
    }

    fn handle_init_local_path(path: &Path) -> Self {
        // for local we start the program off filtering off .files
        // since its a visual hindrance. default ls does not show .files either
        let dents = getdents_from_path(path)
            .expect("Initial path is not a directory")
            .into_iter()
            .filter(|entry | filter_dot_dent(&entry))
            .collect();

        return Self {
            cwd: AnyPath::LocalPath(path.to_path_buf()), // cwd
            dents: dents,
            dents_filterview: None,
            cloud_client: None,
            navigator: AnyNavigator::LocalNavigator,
            show_dotfiles: false, // defaults to not showing dotfiles because its a visual hindrance
            cursor_y: 0,
            view_slice: [0,10], // this will be overridden very quickly
            available_area: [10, 10], // to be reset very soon.
            sig_user_input_exit: false,
            sig_file_selected_exit: false,
            input_handler: InputHandler::new(),
            mode: Mode::Normal,
        }
    }

    pub fn should_exit(&self) -> bool {
        return self.sig_user_input_exit 
        || self.sig_file_selected_exit;
    }

    pub fn update_table_area(&mut self, main_table_area: Rect) {
        let [curr_height, curr_width] = &self.available_area;
        if !(main_table_area.height == *curr_height && main_table_area.width == *curr_width) {
            // update the table area
            self.available_area = [main_table_area.height, main_table_area.width];

            self.recalculate_view_slice();
        }
    }

    pub fn recalculate_renderable_rows(&self) -> u16 {
        let [curr_height, _] = &self.available_area;
        let dents = {
            if self.dents_filterview.is_some() {
                self
                    .dents_filterview
                    .as_ref()
                    .unwrap()
            }else {
                &self.dents
            }
        };

        let max_entries = &dents.len();
        return (curr_height / CELL_HEIGHT)
            .min(*max_entries as u16);
    }

    pub fn recalculate_view_slice(&mut self) {
        let [start, _] = &self.view_slice;
        // let renderable_rows = self.recalculate_renderable_rows();
        self.view_slice = [*start, start+self.recalculate_renderable_rows()];
    }

    pub fn set_cwd(&mut self, new_cwd: AnyPath) {
        if let AnyPath::GSPath(path_str) = &new_cwd {
            // assert ends with /
            if !path_str.ends_with('/') {
                panic!("GCS path must end with / otherwise list_objects logic will fail.");
            }
        } else {};
        self.cwd = new_cwd;
    }
}