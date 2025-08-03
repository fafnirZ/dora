use serde_json::Value;

use crate::library::internal::{node::Node, node_path::{NodePath, NodePathKey}};

#[derive(Debug)]
pub enum AnyNode {
    PrimitiveNode(Value),
    IterableNodes(Vec<Node>),
    NestedNode(Node),
}


#[derive(Debug)]
pub struct ParsedNodePacket {
    pub key: String, // key from json
    pub node: AnyNode, // Node with type information
}


impl AnyNode {
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
