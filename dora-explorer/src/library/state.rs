
// very primitive state right now
// not optimised and not cached.
pub struct ExplorerState{
    pub cwd: String,
    pub children: Vec<String>,
}

impl ExplorerState {
    pub fn new() -> Self {
        Self {
            cwd: "/path/to/a".to_string(),
            children: Vec::new(),
        }
    }
}