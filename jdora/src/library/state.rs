use std::{any::Any, env, fs::File, io::Read, path::{Path, PathBuf}};

use google_cloud_storage::client::Client;
use ratatui::layout::Rect;
use tui_input::Input;

use super::{input::InputHandler, internal::{node::Node, node_path::NodePath, parser::parse_bytes}, mode::Mode, ui::CELL_HEIGHT};


// very primitive state right now
// not optimised and not cached.
pub struct ExplorerState{
    pub root_node_state: Node,
    pub root_node_structure: Vec<(String, NodePath)>,


    // viewslice
    pub view_slice: [u16;2],
    // line cursor (relative?)
    pub cursor_y: u16,
    available_area: [u16;2],
    
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
            root_node_structure: node.get_structures(),
            root_node_state: node, 
            view_slice: [0, 10],
            available_area: [0,10],
            cursor_y: 0_u16,
            input_handler: InputHandler::new(),
            mode: Mode::Normal,
            sig_user_input_exit: false,
        }
    }

    pub fn should_exit(&self) -> bool {
        self.sig_user_input_exit
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
        return *curr_height
    }

    pub fn recalculate_view_slice(&mut self) {
        let [start, _] = &self.view_slice;
        // let renderable_rows = self.recalculate_renderable_rows();
        self.view_slice = [*start, start+self.recalculate_renderable_rows()];
    }

}