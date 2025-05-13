use std::path::PathBuf;

pub enum FileType {
    Dir,
    File,
    Symlink,
}

pub struct DEnt {
    pub ftype: FileType,
    pub path: PathBuf, // may need to change to string to accomodate for gs types.
}

impl DEnt {
    pub fn new(path: PathBuf, ftype: FileType) -> Self {
        Self {
            ftype,
            path,
        }
    }
}
