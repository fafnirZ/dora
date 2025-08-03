use core::panic;
use std::primitive;

use serde_json::{Map, Value};

use crate::library::internal::{any::{AnyNode, ParsedNodePacket}, constants::INDENT_SIZE, node};

use super::node_path::{self, NodePath, NodePathKey};



// i only consider
// Dictionaries as nodes.



#[derive(Debug)]
pub struct Node {
    pub node_path: NodePath,
    pub indent_level: u16,
    pub children: Vec<ParsedNodePacket>,

    // hidden_children:
    pub hidden_children: Vec<NodePathKey>,
}

impl Node {
    pub fn new(val: Value, node_path: NodePath) -> Self {
        let children  = AnyNode::parse(&val, &node_path);
        Self{
            // serde_node: val,
            node_path: node_path.clone(),
            indent_level: (node_path.path.len() as u16),
            children: children,
            hidden_children: Vec::new(),
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

        
        // handling more complex
        // nested cases
        for (idx, node_packet) in self.children.iter().enumerate() {
            let key = &node_packet.key;
            let child_node = &node_packet.node;

            match child_node {
                AnyNode::IterableNodes(nodes) => {
                    let current_node_owned_formatted_string = format!(
                        "{}\"{}\":",
                        self.num_spaces((self.indent_level+1)*INDENT_SIZE),
                        key.clone(),
                    );
                    // TODO handle lists... its gonna be a nightmare
                    if self.hidden_children.contains(&NodePathKey::DictKey(key.clone())) { // TODO rework hidden children
                        let res = format!("{} <collapsed>({} lines) ▲\n", current_node_owned_formatted_string, child_node.calculate_num_lines());
                        results.push(
                            (
                                res,
                                self.node_path.push_and_clone(NodePathKey::DictKey(key.clone()))
                            )
                        );
                    } else {
                        let res = format!("{} [ ▼\n", current_node_owned_formatted_string);
                        results.push(
                            (
                                res,
                                self.node_path.push_and_clone(NodePathKey::DictKey(key.clone()))
                            )
                        );
                        results.extend(AnyNode::get_structures(child_node));
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
                }
                AnyNode::NestedNode(node) => {
                    let current_node_owned_formatted_string = format!(
                        "{}\"{}\":",
                        self.num_spaces((self.indent_level+1)*INDENT_SIZE),
                        key.clone(),
                    );
                    // TODO handle lists... its gonna be a nightmare
                    if self.hidden_children.contains(&NodePathKey::DictKey(key.clone())) { // TODO rework hidden children
                        let res = format!("{} <collapsed>({} lines) ▲\n", current_node_owned_formatted_string, child_node.calculate_num_lines());
                        results.push(
                            (
                                res,
                                self.node_path.push_and_clone(NodePathKey::DictKey(key.clone()))
                            )
                        );
                    } else {
                        let res = format!("{} {{ ▼\n", current_node_owned_formatted_string);
                        results.push(
                            (
                                res,
                                self.node_path.push_and_clone(NodePathKey::DictKey(key.clone()))
                            )
                        );
                        // recursively call children get_structures.
                        let children_structures = child_node.get_structures();
                        for res in children_structures {
                            results.push(res);
                        }
                    }

                }
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