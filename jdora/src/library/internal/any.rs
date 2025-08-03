use serde_json::Value;

use crate::library::internal::{node::Node, node_path::{NodePath, NodePathKey}};

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
