use super::node::Node;

#[derive(Clone, Debug)]
pub struct NodePath {
    pub path: Vec<usize>
}

impl NodePath {
    pub fn new() -> Self {
        Self {
            path: Vec::new()
        }
    }
    pub fn new_with_path(path: Vec<usize>) -> Self {
        Self {
            path: path
        }
    }

    pub fn push_and_clone(&self, idx: usize) -> Self {
        let mut _p = self.path.clone();
        _p.push(idx);

        return Self::new_with_path(_p);
    }


}