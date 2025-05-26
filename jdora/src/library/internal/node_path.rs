use super::node::Node;

#[derive(Clone, Debug)]
pub enum NodePathKey {
    DictKey(String),
    ListIndex(usize),
}

#[derive(Clone, Debug)]
pub struct NodePath {
    pub path: Vec<NodePathKey>
}

impl NodePath {
    pub fn new() -> Self {
        Self {
            path: Vec::new()
        }
    }
    pub fn new_with_path(path: Vec<NodePathKey>) -> Self {
        Self {
            path: path
        }
    }

    pub fn push_and_clone(&self, val: NodePathKey) -> Self {
        let mut _p = self.path.clone();
        _p.push(val);

        return Self::new_with_path(_p);
    }


}