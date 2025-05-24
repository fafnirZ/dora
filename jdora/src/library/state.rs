use std::{any::Any, env, path::{Path, PathBuf}};

use google_cloud_storage::client::Client;
use ratatui::layout::Rect;
use tui_input::Input;

use super::{input::InputHandler, mode::Mode,  ui::CELL_HEIGHT};


// very primitive state right now
// not optimised and not cached.
pub struct ExplorerState{
    pub input_handler: InputHandler,
    pub mode: Mode,
    pub sig_user_input_exit: bool,
}

impl ExplorerState {
    pub fn new(file_path: Option<String>) -> Self {
        // use cwd
        // initial path for testing purposes
        // no remote path unless explicitly arg passed in begins with gs://
        let local_cwd = env::current_dir().unwrap();
        return ExplorerState::handle_init_local_path(&local_cwd);           
    }


    fn handle_init_local_path(path: &Path) -> Self {

        return Self {
            input_handler: InputHandler::new(),
            mode: Mode::Normal,
            sig_user_input_exit: false,
        }
    }

    pub fn should_exit(&self) -> bool {
        self.sig_user_input_exit
    }

}