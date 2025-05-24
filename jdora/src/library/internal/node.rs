use serde_json::{Map, Value};


// i only consider
// Dictionaries as nodes.
pub struct Node {
    serde_node: Value, // forwards to serde node
}

impl Node {
    pub fn new(val: Value) -> Option<Self> {
        if !matches!(val, Value::Object(_)) {
            return None
        }
        Some(Self{serde_node: val})
    }

    pub fn parse(&self) {
        if let Value::Object(map) = &self.serde_node {
            // node primitives
            let mut primitives: Vec<(String, Value)> = Vec::new();
            let mut children: Vec<(String, Node)> = Vec::new();

            for (key, val) in map.iter() {
                match val {
                    Value::Object(_) => {
                        children.push(
                            (key.to_string(), Node::new(val.clone()).unwrap())
                        )
                    }
                    _ => {
                        primitives.push(
                            (key.to_string(), val.clone())
                        )
                    }
                }
            }
        }
    }

    // // filters out children nodes
    // pub fn get_node_primitives(&self) -> Map<String, Value> {

    // }
    // pub fn get_children_nodes(&self) -> Vec<Node> {

    // }
}

