use std::primitive;

use serde_json::{Map, Value};


const INDENT_SIZE: u16 = 4;

// i only consider
// Dictionaries as nodes.

#[derive(Debug)]
pub struct Node {
    serde_node: Value, // forwards to serde node
    indent_level: u16,
    primitives: Vec<(String, Value)>,
    children: Vec<(String, Node)>,

    // hidden_children:
    pub hidden_children: Vec<u16>,
}

impl Node {
    pub fn new(val: Value, indent_level: u16) -> Option<Self> {
        if !matches!(val, Value::Object(_)) {
            return None
        }

        let (primitives, children) = Node::parse(&val, indent_level);
        Some(Self{
            serde_node: val,
            indent_level: indent_level,
            primitives: primitives,
            children: children,
            hidden_children: Vec::new(),
        })
    }

    pub fn parse(serde_node: &Value, indent_level: u16) -> (
        Vec<(String, Value)>, // primitives
        Vec<(String, Node)>,  // nested nodes 
    ){
        if let Value::Object(map) = &serde_node {
            // node primitives
            let mut primitives: Vec<(String, Value)> = Vec::new();
            let mut children: Vec<(String, Node)> = Vec::new();

            for (key, val) in map.iter() {
                match val {
                    Value::Object(_) => {
                        children.push(
                            (
                                key.to_string(), 
                                Node::new(val.clone(), indent_level+1).unwrap()
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


        // open brace
        // result += &self.num_spaces(self.indent_level*INDENT_SIZE);
        result += "{\n";

        // print primitives first
        for prim_attr in self.primitives.iter() {
            let (key, val) = prim_attr.clone();
            result += &self.num_spaces((self.indent_level+1)*INDENT_SIZE);
            result += &format!("\"{}\":{}", key, val.to_string());
            result += ",\n"
        }

        // print children
        for (idx, child ) in self.children.iter().enumerate() {
            if self.hidden_children.contains(&(idx as u16)) {
                continue;
            }
            let (key, chld) = child;
            result += &self.num_spaces((self.indent_level+1)*INDENT_SIZE);
            result += &format!("\"{}\":", key);
            result += &chld.pprint();
        }

        result += &self.num_spaces(self.indent_level*INDENT_SIZE);
        result += "}\n";
        
        result
    }

    fn num_spaces(&self, n: u16) -> String {
        " ".repeat(n as usize).to_string()
    }

    // calculate how many lines it this node will consume
    fn calculate_num_lines(&self) -> u16 {
        
        // TODO handle hidden children

        let primitive_len = self.primitives.len() as u16;
        let children_len = self
            .children
            .iter()
            .fold(0 as u16, |acc, &(_, ref child)| acc + child.calculate_num_lines());
        primitive_len+children_len
    }
}

