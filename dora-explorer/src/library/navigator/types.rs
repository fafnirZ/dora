use std::{cmp, path::PathBuf};

use super::traits::AnyPath;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub enum FileType {
    Dir,
    File,
    Symlink,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub struct DEnt {
    pub ftype: FileType,
    pub path: AnyPath, // may need to change to string to accomodate for gs types.
}

impl DEnt {
    pub fn new(path: AnyPath, ftype: FileType) -> Self {
        Self {
            ftype,
            path,
        }
    }
}


impl Ord for DEnt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_fname = self.path.file_name().unwrap();
        let other_fname = other.path.file_name().unwrap();

        return self_fname.chars().cmp(other_fname.chars());
    }
}
