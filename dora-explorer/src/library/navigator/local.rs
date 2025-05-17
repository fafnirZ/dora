use std::{any::Any, f32::consts::E, path::{Path, PathBuf}};

use itertools::sorted;

use crate::library::{errors::ExplorerError, ExplorerState};

use super::{traits::{AnyPath, Navigator}, types::{DEnt, FileType}};


pub struct LocalNavigator{}


impl Navigator for LocalNavigator {

    fn go_out_of_folder(state: &mut ExplorerState) -> Result<(), ExplorerError> {

        if let AnyPath::LocalPath(cwd) = &state.cwd {
            let new_path = match cwd.parent() {
                Some(res) => res,
                None => return Ok(()), // exits function without error? just doesnt do much
            };
            
            // check if the new path is a dir 
            // propagates error early and exits fn
            getdents_from_path(&new_path)?;
            
            // updating cwd
            state.cwd = AnyPath::LocalPath(new_path.to_path_buf());

            // refresh dents
            Self::refresh_d_ents(state)?;

            // refresh cursor
            state.cursor_y = 0;

            // refresh view slice
            let renderable_rows = state.recalculate_renderable_rows();
            state.view_slice = [0, renderable_rows];

            Ok(())
        } else {
            return Err(ExplorerError::NotALocalPath("Expected a local path.".to_string()))
        }
        
    }

    fn go_into_folder(state: &mut ExplorerState) -> Result<(), ExplorerError> {
        
        if let AnyPath::LocalPath(cwd) = &state.cwd {
            let cursor_pos = *&state.cursor_y;
            let absolute_pos = &state.view_slice[0] + cursor_pos;
            if let AnyPath::LocalPath(selected_dir) = &state.dents[absolute_pos as usize].path {
                let new_path = cwd.join(selected_dir);

                // check if the new path is a dir 
                // propagates error early and exits fn
                getdents_from_path(&new_path)?;
                
                // updating cwd
                state.cwd = AnyPath::LocalPath(new_path);

                // refresh dents
                Self::refresh_d_ents(state)?;

                // refresh cursor
                state.cursor_y = 0;

                // refresh view slice
                let renderable_rows = state.recalculate_renderable_rows();
                state.view_slice = [0, renderable_rows];

                Ok(())
            } else {
                return Err(ExplorerError::NotALocalPath("Expected a local path.".to_string()))
            }
        } else {
            return Err(ExplorerError::NotALocalPath("Expected a local path.".to_string()))
        }
    }

    fn refresh_d_ents(state: &mut ExplorerState) -> Result<(), ExplorerError> {
        if let AnyPath::LocalPath(cwd) = &state.cwd {
            state.dents = {
                if !state.show_dotfiles {
                    getdents_from_path(cwd)?
                    .into_iter()
                    .filter(|entry| filter_dot_dent(&entry))
                    .collect()
                }
                else {
                    getdents_from_path(cwd)?
                }
            };
            Ok(())
        } else {
            return Err(ExplorerError::NotALocalPath("Expected a local path.".to_string()))
        }
    }
}



// local path implementation
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
                DEnt::new(
                    AnyPath::LocalPath(path), 
                    ftype
                )
            )
        }
    }

    // sort dents
    let sorted_dents = sorted(dents).collect();

    Ok(sorted_dents)
}

// fn to be applied to a filter op
// returns true if not a .file
pub fn filter_dot_dent(entry: &DEnt) -> bool {
    let fname = entry
        .path
        .file_name()
        .unwrap_or("");

    if fname.starts_with(".") {
        return false
    }
    true
}