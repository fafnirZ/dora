use core::panic;
use std::primitive;

use serde_json::{Map, Value};

use crate::library::internal::node;

use super::node_path::{self, NodePath, NodePathKey};


const INDENT_SIZE: u16 = 4;

// i only consider
// Dictionaries as nodes.

#[derive(Debug)]
pub enum AnyNode {
    PrimitiveNode(Value),
    IterableNodes(Vec<Node>),
    NestedNode(Node),
}

impl AnyNode {
    pub fn calculate_num_lines(&self) -> u16 {
        match self {
            AnyNode::PrimitiveNode(_) => {
                1 as u16
            },
            AnyNode::IterableNodes(nodes) => {
                0 as u16 // TODO
            },
            AnyNode::NestedNode(node) => {
                node.calculate_num_lines()
            }
        }
    }

    pub fn toggle_hide_child(&mut self, child: &NodePathKey) {
        match self {
            AnyNode::PrimitiveNode(_) => {
                // do nothing
            },
            AnyNode::IterableNodes(nodes) => {
                // do nothing for now
            },
            AnyNode::NestedNode(node) => {
                if let Some(idx) = node.hidden_children.iter().position(|item| item==child) {
                    node.hidden_children.remove(idx);
                } else {
                    node.hidden_children.push(child.clone());
                }
            }
        }
    }

    pub fn get_structures(&self) -> Vec<(String, NodePath)> {
        let mut structures: Vec<(String, NodePath)> = Vec::new();
        match self {
            AnyNode::PrimitiveNode(_) => {
                panic!("Cannot call this function on primitive.")
            }
            AnyNode::IterableNodes(nodes) => {
                // TODO
            }
            AnyNode::NestedNode(node) => {
                structures.extend(node.get_structures());
            }
        }
        structures
    }

    pub fn pprint(&self) -> String {
        let mut curr_str = String::new();
        match self {
            AnyNode::PrimitiveNode(_) => {
                panic!("Cannot call this function on primitive.")
            }
            AnyNode::IterableNodes(nodes) => {
                //
            }
            AnyNode::NestedNode(node)  => {
                curr_str += &node.pprint();
            }
        }
        curr_str
    }
}

#[derive(Debug)]
pub struct ParsedNodePacket {
    key: String, // key from json
    node: AnyNode, // Node with type information
}


#[derive(Debug)]
pub struct Node {
    // serde_node: Value, // forwards to serde node
    pub node_path: NodePath,
    pub indent_level: u16,
    // pub primitives: Vec<(String, Value)>, // primitive attributes
    // NOTE i understand I don't handle lists well....at all right now...
    pub children: Vec<ParsedNodePacket>,

    // hidden_children:
    pub hidden_children: Vec<NodePathKey>,
}

impl Node {
    pub fn new(val: Value, node_path: NodePath) -> Self {
        // if !matches!(val, Value::Object(_)) {
        //     return None
        // }

        let children  = Node::parse(&val, &node_path);
        Self{
            // serde_node: val,
            node_path: node_path.clone(),
            indent_level: (node_path.path.len() as u16),
            children: children,
            hidden_children: Vec::new(),
        }
    }

    pub fn parse(serde_node: &Value, node_path: &NodePath) -> Vec<ParsedNodePacket> {
        if let Value::Object(map) = &serde_node {
            let mut children: Vec<ParsedNodePacket> = Vec::new();

            for (key, val) in map.iter() {
                match val {
                    Value::Object(_) => {
                        children.push(
                            ParsedNodePacket{
                                key: key.to_string(), 
                                node: AnyNode::NestedNode(Node::new(
                                    val.clone(), 
                                    node_path.push_and_clone(
                                        NodePathKey::DictKey(key.to_string())
                                    ),
                                )),
                            }
                        )
                    }
                    Value::Array(_) => {
                        // TODO 
                        //     ParsedNodePacket{
                        //         key: key.to_string(), 
                        //         node: AnyNodeNode::new(
                        //             val.clone(), 
                        //             node_path.push_and_clone(
                        //                 NodePathKey::DictKey(key.to_string())
                        //             ),
                        //             NodeType::IterableNodes,
                        //         ),
                        //     }
                        // ) 
                    }
                    _ => {
                        children.push(
                            ParsedNodePacket{
                                key: key.to_string(), 
                                node: AnyNode::PrimitiveNode(val.clone()),
                            }
                        )
                    }
                }
            }
            return children
        } else {
            panic!("parse failed? node is not an object")
        }
    }

    ///
    /// for every line which will be sent to pprint
    /// associate the NodePath associated with this line.
    pub fn get_structures(&self) -> Vec<(String, NodePath)> {

        let mut results: Vec<(String, NodePath)> = Vec::new();

        // NOTE: only do this for root nodepath
        if self.indent_level == 0 {
            // open brace
            let open_bracket_str = "{\n".to_string();
            results.push(
                (
                    open_bracket_str, // this belongs to this node.
                    self.node_path.clone()
                )
            );
        }

        // // print primitives first
        // for prim_attr in self.primitives().iter() {
        //     let key = prim_attr.key.clone();
        //     if let AnyNode::PrimitiveNode(value) = prim_attr.node {
        //         let formatted_str = format!(
        //             "{}\"{}\":{},\n",
        //             self.num_spaces((self.indent_level+1)*INDENT_SIZE),
        //             key.clone(),
        //             value.to_string(),
        //         );
        //         result.push(
        //             (
        //                 formatted_str,
        //                 self.node_path.push_and_clone(NodePathKey::DictKey(key.clone()))
        //             )
        //         )
        //     } else {
        //         panic!("should be unreachable...")
        //     }
        // }
        
        // handling more complex
        // nested cases
        for (idx, node_packet) in self.children.iter().enumerate() {
            // let current_node_owned_formatted_string = format!(
            //     "{}\"{}\":",
            //     self.num_spaces((self.indent_level+1)*INDENT_SIZE),
            //     key.clone(),
            // );
            // // TODO handle lists... its gonna be a nightmare
            // if self.hidden_children.contains(&NodePathKey::DictKey(key.clone())) { // TODO rework hidden children
            //     let res = format!("{} <collapsed>({} lines) ▲\n", current_node_owned_formatted_string, child_node.calculate_num_lines());
            //     result.push(
            //         (
            //             res,
            //             self.node_path.push_and_clone(NodePathKey::DictKey(key.clone()))
            //         )
            //     );
            // } else {
            //     let res = format!("{} {{ ▼\n", current_node_owned_formatted_string);
            //     result.push(
            //         (
            //             res,
            //             self.node_path.push_and_clone(NodePathKey::DictKey(key.clone()))
            //         )
            //     );
            //     // recursively call children get_structures.
            //     let children_structures = child_node.get_structures();
            //     for res in children_structures {
            //         result.push(res);
            //     }
            // }

            let key = &node_packet.key;
            let child_node = &node_packet.node;

            match child_node {
                AnyNode::IterableNodes(nodes) => {}
                AnyNode::NestedNode(node) => {}
                AnyNode::PrimitiveNode(value) => {
                    let result = self.get_structures_primitive(key.clone(), value.clone());
                    results.push(result);
                }
            }


        } 

        // print closing bracket
        let closing_bracket_str = format!(
            "{}}}\n",
            self.num_spaces(self.indent_level*INDENT_SIZE) 
        );
        results.push(
            (
                closing_bracket_str,
                self.node_path.clone(),
            )
        );
        
        results
    }

    pub fn get_structures_primitive(&self, key: String, value: Value) -> (String, NodePath)  {
        let formatted_str = format!(
            "{}\"{}\":{},\n",
            self.num_spaces((self.indent_level+1)*INDENT_SIZE),
            key.clone(),
            value.to_string(),
        );
        
        return(
            formatted_str,
            self.node_path.push_and_clone(NodePathKey::DictKey(key.clone()))
        )
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

        let primitive_len = self.primitives().len() as u16;
        let children_len = self
            .children
            .iter()
            .fold(0 as u16, |acc, packet| acc + packet.node.calculate_num_lines());
        bracket_lines+primitive_len+children_len
    }

    pub fn get_child(&self, key: &NodePathKey) -> Option<&AnyNode> {
        match key {
            NodePathKey::DictKey(k) => {
                for node_packet in &self.children {
                    let key = &node_packet.key;
                    let node = &node_packet.node;
                    if *key == *k {
                        return Some(&node);
                    }
                }
                None
            }
            _ => {
                // not implemented
                panic!("not implemented");
            }
        }
    }
    pub fn get_child_mut(&mut self, key: &NodePathKey) -> Option<&mut AnyNode> {
        match key {
            NodePathKey::DictKey(k) => {
                for node_packet in self.children.iter_mut() {
                    let key = &node_packet.key;
                    let node = &mut node_packet.node;
                    if *key == *k {
                        return Some(node);
                    }
                }
                None
            }
            _ => {
                // not implemented
                panic!("not implemented");
            }
        }
    }

    pub fn primitives(&self) -> Vec<&ParsedNodePacket> {
        return self.children.iter()
                .filter(|p| 
                    matches!(p.node, AnyNode::PrimitiveNode(_))
                )
                .map(|val| val)
                .collect();
    }
    pub fn nested_nodes(&self) -> Vec<&ParsedNodePacket> {
        return self.children.iter()
                .filter(|p| 
                    matches!(p.node,AnyNode::NestedNode(_))
                )
                .map(|val| val)
                .collect();
    }



    // pub fn toggle_hide_child(&mut self, child: &NodePathKey) {
    //     if let Some(idx) = self.hidden_children.iter().position(|item| item==child) {
    //         self.hidden_children.remove(idx);
    //     } else {
    //         self.hidden_children.push(child.clone());
    //     }
    // }

}



pub fn try_resolve_node_path<'a>(root_node: &'a AnyNode, node_path: &NodePath) -> Option<&'a AnyNode> {
    let cur_any_node = root_node;
    match cur_any_node {
        AnyNode::PrimitiveNode(_) => {
            panic!("root node is a primitive????")
        },
        AnyNode::IterableNodes(nodes) => {
            // pass   
            None
        },
        AnyNode::NestedNode(cur_node) => {
            for path_key in &node_path.path {
                if let Some(new_node) = cur_node.get_child(&path_key) {
                    return Some(new_node)
                } else {
                    return None
                }
            }
            None
        }
    }
}

// returns a mutable reference, this is more necesary when we want to 
// update the state of the resolved path
// the other one is more useful when we just wanna perform queries
pub fn try_resolve_node_path_mut<'a>(root_node: &'a mut AnyNode, node_path: &NodePath) -> Option<&'a mut AnyNode> {
    let cur_any_node = root_node;
    match cur_any_node {
        AnyNode::PrimitiveNode(_) => {
            panic!("root node is a primitive????")
        },
        AnyNode::IterableNodes(nodes) => {
            // pass   
            None
        },
        AnyNode::NestedNode(cur_node) => {
            for path_key in &node_path.path {
                if let Some(new_node) = cur_node.get_child_mut(&path_key) {
                    return Some(new_node)
                } else {
                    return None
                }
            }
            None
        }
    }
}