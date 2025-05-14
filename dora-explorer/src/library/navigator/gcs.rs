use std::{any::Any, f32::consts::E, path::{Path, PathBuf}};

use crate::library::{errors::ExplorerError, ExplorerState};

use super::{traits::{AnyPath, Navigator}, types::{DEnt, FileType}};


pub struct GCSNavigator{}


impl Navigator for GCSNavigator {

    fn go_out_of_folder(state: &mut ExplorerState) -> Result<(), ExplorerError> {

        if let AnyPath::GSPath(cwd) = &state.cwd {
            Ok(())
        } else {
            return Err(ExplorerError::NotARemotePath("Expected a local path.".to_string()))
        }
        
    }

    fn go_into_folder(state: &mut ExplorerState) -> Result<(), ExplorerError> {
        
                if let AnyPath::GSPath(cwd) = &state.cwd {
            Ok(())
        } else {
            return Err(ExplorerError::NotARemotePath("Expected a local path.".to_string()))
        }
    }

    fn refresh_d_ents(state: &mut ExplorerState) -> Result<(), ExplorerError> {
        if let AnyPath::GSPath(cwd) = &state.cwd {
            Ok(())
        } else {
            return Err(ExplorerError::NotARemotePath("Expected a local path.".to_string()))
        }
    }
}



// local path implementation
pub fn getdents_from_path(path: &Path) -> Result<Vec<DEnt>, ExplorerError>{

    Ok(())
}