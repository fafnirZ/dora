pub struct NodePath {
    path: Vec<usize>
}

impl NodePath {
    pub fn new() -> Self {
        Self {
            path: Vec::new()
        }
    }
}