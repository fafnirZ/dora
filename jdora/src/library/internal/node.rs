use std::primitive;

use serde_json::{Map, Value};

use super::node_path::{self, NodePath, NodePathKey};


const INDENT_SIZE: u16 = 4;

// i only consider
// Dictionaries as nodes.

#[derive(Debug)]
pub struct Node {
    // serde_node: Value, // forwards to serde node
    pub node_path: NodePath,
    pub indent_level: u16,
    pub primitives: Vec<(String, Value)>, // primitive attributes
    // NOTE i understand I don't handle lists well....at all right now...
    pub children: Vec<(String, Node)>,

    // hidden_children:
    pub hidden_children: Vec<u16>,
}

impl Node {
    pub fn new(val: Value, node_path: NodePath) -> Option<Self> {
        if !matches!(val, Value::Object(_)) {
            return None
        }

        let (primitives, children) = Node::parse(&val, &node_path);
        Some(Self{
            // serde_node: val,
            node_path: node_path.clone(),
            indent_level: (node_path.path.len() as u16),
            primitives: primitives,
            children: children,
            hidden_children: Vec::new(),
        })
    }

    pub fn parse(serde_node: &Value, node_path: &NodePath) -> (
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
                                Node::new(
                                    val.clone(), 
                                    node_path.push_and_clone(
                                        NodePathKey::DictKey(key.to_string())
                                    )
                                ).unwrap()
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

    ///
    /// for every line which will be sent to pprint
    /// associate the NodePath associated with this line.
    pub fn get_structures(&self) -> Vec<(String, NodePath)> {
        let mut result: Vec<(String, NodePath)> = Vec::new();

        // open brace
        let open_bracket_str = "{\n".to_string();
        result.push(
            (
                open_bracket_str, // this belongs to this node.
                self.node_path.clone()
            )
        );

        // print primitives first
        for prim_attr in self.primitives.iter() {
            let (key, val) = prim_attr.clone();
            let formatted_str = format!(
                "{}\"{}\":{},\n",
                self.num_spaces((self.indent_level+1)*INDENT_SIZE) ,
                key.clone(),
                val.to_string(),
            );
            result.push(
                (
                    formatted_str,
                    self.node_path.push_and_clone(NodePathKey::DictKey(key.clone()))
                )
            )
        }

        for (idx, child ) in self.children.iter().enumerate() {
            let (key, chld) = child;
            let current_node_owned_formatted_string = format!(
                "{}\"{}\":",
                self.num_spaces((self.indent_level+1)*INDENT_SIZE),
                key.clone(),
            );
            if self.hidden_children.contains(&(idx as u16)) { // TODO rework hidden children
                let res = format!("{} <collapsed>({} lines)\n", current_node_owned_formatted_string, chld.calculate_num_lines());
                result.push(
                    (
                        current_node_owned_formatted_string,
                        self.node_path.push_and_clone(NodePathKey::DictKey(key.clone()))
                    )
                );
            } else {
                let res = format!("{} {{\n", current_node_owned_formatted_string);
                result.push(
                    (
                        current_node_owned_formatted_string,
                        self.node_path.push_and_clone(NodePathKey::DictKey(key.clone()))
                    )
                );
            }

            // recursively call children get_structures.
            let children_structures = chld.get_structures();
            for res in children_structures {
                result.push(res);
            }
        } 

        // print closing bracket
        let closing_bracket_str = format!(
            "{}}}\n",
            self.num_spaces(self.indent_level*INDENT_SIZE) 
        );
        result.push(
            (
                closing_bracket_str,
                self.node_path.clone(),
            )
        );
        
        result
    }

    pub fn pprint(&self) -> String {
        let mut result = String::new();
        for (str, _) in self.get_structures() { // unoptimised.
            result += &str;
        }
        result
    }


    fn num_spaces(&self, n: u16) -> String {
        " ".repeat(n as usize).to_string()
    }

    // calculate how many lines it this node will consume
    // this just counts size of primitive and recursively adds up children offset lengths.
    pub fn calculate_num_lines(&self) -> u16 {
        
        // TODO handle hidden children
        let bracket_lines = 2_u16;

        let primitive_len = self.primitives.len() as u16;
        let children_len = self
            .children
            .iter()
            .fold(0 as u16, |acc, &(_, ref child)| acc + child.calculate_num_lines());
        bracket_lines+primitive_len+children_len
    }
    
    // // TODO: need to think deeply
    // // about the maths and calculations
    // // for this function and the calculate_num_lines_function
    // pub fn build_children_line_boundaries(&self, current_line_pos: u16, curr_node_path: &NodePath) -> Vec<(NodePath, (u16,u16))>{
    //     if self.children.len() == 0 { // is leaf node
    //         let mut v = Vec::new();
    //         v.push(
    //             (
    //                 curr_node_path.clone(), 
    //                 (current_line_pos, current_line_pos+self.calculate_num_lines()-1)
    //             )
    //         );
    //         return v;
    //     }

    //     let mut v = Vec::new();
    //     let open_bracket_offset = 1_u16;
    //     let mut line_pos_offset = current_line_pos 
    //         + (self.primitives.len() as u16)
    //         + open_bracket_offset;

    //     for (idx, (_, child)) in self.children.iter().enumerate() {
    //         let new_node_path = curr_node_path.push_and_clone(idx);
    //         let child_boundaries = child.build_children_line_boundaries(
    //             line_pos_offset, // update line position offsets
    //             &new_node_path
    //         ); 
    //         for child_boundary in child_boundaries {
    //             v.push(child_boundary);
    //         }
    //         let child_total_len = child.calculate_num_lines();
    //         line_pos_offset += child_total_len;
    //     }
    //     v.push(
    //         (curr_node_path.clone(), (current_line_pos, current_line_pos+(self.calculate_num_lines()-1)))
    //     );
    //     return v;
    // }
}

