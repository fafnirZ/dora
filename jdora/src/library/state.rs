use std::{any::Any, env, fs::File, io::Read, path::{Path, PathBuf}};

use google_cloud_storage::client::Client;
use ratatui::layout::Rect;
use tui_input::Input;

use super::{input::InputHandler, internal::{node::Node, parser::parse_bytes}, mode::Mode, ui::CELL_HEIGHT};


// very primitive state right now
// not optimised and not cached.
pub struct ExplorerState{
    pub node_state: Node,
    pub cursor_y: u16,
    // todo view slice
    
    pub input_handler: InputHandler,
    pub mode: Mode,
    pub sig_user_input_exit: bool,
}

impl ExplorerState {
    pub fn new(file_path: Option<String>) -> Self {
        return ExplorerState::handle_init_local_path(Path::new(&file_path.unwrap()));           
    }


    fn handle_init_local_path(path: &Path) -> Self {
        
        // read file
        let mut file = File::open(path).unwrap();
        let mut contents:Vec<u8> = Vec::new();
        
        file.read_to_end(&mut contents)
            .unwrap();

        let node = parse_bytes(&contents);

        return Self {
            node_state: node, 
            cursor_y: 0_u16,
            input_handler: InputHandler::new(),
            mode: Mode::Normal,
            sig_user_input_exit: false,
        }
    }

    pub fn should_exit(&self) -> bool {
        self.sig_user_input_exit
    }

}