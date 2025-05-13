use std::path::{Path, PathBuf};

use crate::library::ExplorerState;

pub fn go_out_of_folder(state: &mut ExplorerState) {
    let cwd = &state.cwd;

    let new_path = match cwd.parent() {
        Some(res) => res,
        None => return, // exits function
    };
    
    // updating cwd
    state.cwd = new_path.to_path_buf();

    // refresh dents
    refresh_d_ents(state);
}

pub fn go_into_folder(state: &mut ExplorerState) {
    let cwd = &state.cwd;
    let cursor_pos = *(&state.cursor_y) as usize;
    let selected_dir = &state.dents[cursor_pos];

    let new_path = cwd.join(selected_dir);
    
    // updating cwd
    state.cwd = new_path;

    // refresh dents
    refresh_d_ents(state);
}

fn refresh_d_ents(state: &mut ExplorerState) {
    let cwd = &state.cwd;
    state.dents = getdents_from_path(cwd);
}

pub fn getdents_from_path(path: &Path) -> Vec<PathBuf>{
    // linux naming...
    let mut dents = Vec::new();
    for entry in path.read_dir().expect("Well this path should exist..") {
        if let Ok(entry) = entry {
            dents.push(
                entry.path()
            )
        }
    }
    dents
}