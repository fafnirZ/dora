use std::{env, path::{Path, PathBuf}};

use super::navigator::local::getdents_from_path;


// very primitive state right now
// not optimised and not cached.
pub struct ExplorerState{
    pub cwd: PathBuf,
    pub dents: Vec<PathBuf>, // directory entries

    // visual states
    pub cursor_y: u16,
}

impl ExplorerState {
    pub fn new() -> Self {
        // initial path for testing purposes
        let cwd = &env::current_dir().unwrap();
        let path = cwd.as_path();
        let dents = getdents_from_path(&path); 
        Self {
            cwd: path.to_path_buf(), // cwd
            dents: dents,
            cursor_y: 0,
        }
    }
}