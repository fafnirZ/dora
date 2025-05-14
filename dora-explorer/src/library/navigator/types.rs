use std::path::PathBuf;

use super::traits::AnyPath;

pub enum FileType {
    Dir,
    File,
    Symlink,
}

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
