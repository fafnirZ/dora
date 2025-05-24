use serde_json::{Map, Value};


const INDENT_SIZE: u16 = 4;

// i only consider
// Dictionaries as nodes.

#[derive(Debug)]
pub struct Node {
    serde_node: Value, // forwards to serde node
    indent_level: u16,
}

impl Node {
    pub fn new(val: Value, indent_level: u16) -> Option<Self> {
        if !matches!(val, Value::Object(_)) {
            return None
        }
        Some(Self{
            serde_node: val,
            indent_level: indent_level,
        })
    }

    pub fn parse(&self) -> (
        Vec<(String, Value)>, // primitives
        Vec<(String, Node)>,  // nested nodes 
    ){
        if let Value::Object(map) = &self.serde_node {
            // node primitives
            let mut primitives: Vec<(String, Value)> = Vec::new();
            let mut children: Vec<(String, Node)> = Vec::new();

            for (key, val) in map.iter() {
                match val {
                    Value::Object(_) => {
                        children.push(
                            (
                                key.to_string(), 
                                Node::new(val.clone(), self.indent_level+1).unwrap()
                            )
                        )
                    }
                    _ => {
                        primitives.push(
                            (key.to_string(), val.clone())
                        )
                    }
                }
            }
            return (
                primitives,
                children,
            )
        } else {
            panic!("parse failed? node is not an object")
        }
    }

    pub fn pprint(&self) -> String {
        let mut result = String::new();
        let (
            primitive,
            nested_children,
        ) = self.parse();

        // open brace
        result += &self.num_spaces(self.indent_level*INDENT_SIZE);
        result += "{";

        // print primitives first
        for prim_attr in primitive.iter() {
            let (key, val) = prim_attr.clone();
            result += &self.num_spaces((self.indent_level+1)*INDENT_SIZE);
            result += &format!("\"{}\":\"{}\"\n", key, val);
        }
        
        result
    }

    fn num_spaces(&self, n: u16) -> String {
        " ".repeat(n as usize).to_string()
    }

}

