use std::path::PathBuf;

pub enum FileType {
    Dir,
    File,
    Symlink,
}

pub struct DEnt {
    ftype: FileType,
    path: PathBuf,
}

impl DEnt {
    pub fn new(path: PathBuf) -> Self {
        // auto determines type by querying

    }
}
