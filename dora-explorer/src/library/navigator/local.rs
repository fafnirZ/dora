use std::{path::{Path, PathBuf}};

use crate::library::{errors::ExplorerError, ExplorerState};

use super::types::{DEnt, FileType};

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

    // refresh cursor
    state.cursor_y = 0;

    // refresh view slice
    let renderable_rows = state.recalculate_renderable_rows();
    state.view_slice = [0, renderable_rows];
}

pub fn go_into_folder(state: &mut ExplorerState) {
    let cwd = &state.cwd;
    let cursor_pos = *&state.cursor_y;
    let absolute_pos = &state.view_slice[0] + cursor_pos;
    let selected_dir = &state.dents[absolute_pos as usize]
        .path;

    let new_path = cwd.join(selected_dir);
    
    // updating cwd
    state.cwd = new_path;

    // refresh dents
    refresh_d_ents(state);

    // refresh cursor
    state.cursor_y = 0;

    // refresh view slice
    let renderable_rows = state.recalculate_renderable_rows();
    state.view_slice = [0, renderable_rows]
}

fn refresh_d_ents(state: &mut ExplorerState) -> Result<(), ExplorerError> {
    let cwd = &state.cwd;
    state.dents = getdents_from_path(cwd)?;
    Ok(())
}

pub fn getdents_from_path(path: &Path) -> Result<Vec<DEnt>, ExplorerError>{
    // linux naming...
    let mut dents = Vec::new();
    let dir_iter = path.read_dir()
        .map_err(|e| ExplorerError::NotADirectoryError(e.to_string()))?;

    for entry in dir_iter {
        if let Ok(entry) = entry {
            let path = entry.path();
            
            let ftype = {
                if path.is_dir() {
                    FileType::Dir
                } else if path.is_file() {
                    FileType::File
                } else if path.is_symlink() {
                    FileType::Symlink
                } else {
                    panic!("Invalid file type, this should never be reachable?")
                }
            };

            dents.push(
                DEnt::new(path, ftype)
            )
        }
    }
    Ok(dents)
}