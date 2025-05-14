use std::{ffi::OsStr, path::PathBuf};


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
}

fn extract_file_name_str(path_str: &str) -> Option<&str> {
    path_str.rfind('/').map(|index| &path_str[index + 1..])
}