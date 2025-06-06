use super::node::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum NodePathKey {
    DictKey(String),
    ListIndex(usize),
}

impl NodePathKey {
    pub fn to_string(&self) -> String {
        match self {
            Self::DictKey(val) => val.clone(),
            Self::ListIndex(val) => val.to_string(),
        }
    }
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

    pub fn to_string(&self) -> String {
        let mut res: Vec<String> = Vec::new();
        for val in &self.path {
            res.push(val.to_string());
        }
        res.join("/")
    }

    pub fn parent(&self) -> NodePath {
        let mut new_path = self
        .path
        .clone();  
        
        // mutate clone
        new_path.pop();

        NodePath::new_with_path(new_path)
    }

    pub fn leaf(&self) -> Option<NodePathKey> {
        let mut path_shadow = self.path.clone();
        
        path_shadow.pop()
    }


}