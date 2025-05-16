use std::{ffi::OsStr, path::PathBuf};

use crate::library::{errors::ExplorerError, ExplorerState};

pub enum AnyNavigator {
    LocalNavigator,
    GCSNavigator,
}

pub trait Navigator {
    fn go_out_of_folder(state: &mut ExplorerState) -> Result<(), ExplorerError>;
    fn go_into_folder(state: &mut ExplorerState) -> Result<(), ExplorerError>;
    fn refresh_d_ents(state: &mut ExplorerState) -> Result<(), ExplorerError>;
}



#[derive(Debug, Clone)]
pub enum AnyPath {
    LocalPath(PathBuf),
    GSPath(String),
}

impl AnyPath {
    pub fn to_str(&self) -> Option<&str> {
        match self {
            AnyPath::LocalPath(path) => path.to_str(),
            AnyPath::GSPath(path) => Some(path),
        } 
    }

    pub fn file_name(&self) -> Option<&str> {
        match self {
            AnyPath::LocalPath(path) => {
                Some(path
                .file_name()
                .unwrap_or(OsStr::new("Invalid FileName"))
                .to_str()
                .expect("Invalid file name"))
            },
            AnyPath::GSPath(path_str) => {
                extract_file_name_str(path_str)
            }
        }
    }

    pub fn ensure_trailing_slash(path: String) -> String {
        if path.ends_with('/') {
            path
        } else {
            format!("{}/", path)
        }
    }
}

fn extract_file_name_str(path_str: &str) -> Option<&str> {
    path_str.rfind('/').map(|index| &path_str[index + 1..])
}