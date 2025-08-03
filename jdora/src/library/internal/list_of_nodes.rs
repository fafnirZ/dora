use std::any::Any;

use serde_json::Value;

use crate::library::internal::{any::{AnyNode, ParsedNodePacket}, constants::INDENT_SIZE, node_path::{NodePath, NodePathKey}};


#[derive(Debug)]
pub struct ListOfNodes {
    pub node_path: NodePath,
    pub indent_level: u16,

    pub children: Vec<ParsedNodePacket>,
    // hidden_children:
    pub hidden_children: Vec<NodePathKey>,
}


impl ListOfNodes {
    pub fn new(val: Value, node_path: NodePath) -> Self {
        let children  = AnyNode::parse(&val, &node_path);
        Self{
            node_path: node_path.clone(),
            indent_level: (node_path.path.len() as u16),
            children: children,
            hidden_children: Vec::new(),
        }
    }

    /// for every line which will be sent to pprint
    /// associate the NodePath associated with this line.
    pub fn get_structures(&self) -> Vec<(String, NodePath)> {

        let mut results: Vec<(String, NodePath)> = Vec::new();

        // NOTE: only do this for root nodepath
        if self.indent_level == 0 {
            // open brace
            let open_bracket_str = "[\n".to_string();
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
                    // do nothing here, its to be handled by AnyNode
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
                        // recursively call children get_structures.
                        let children_structures = child_node.get_structures();
                        for res in children_structures {
                            results.push(res);
                        }
                    }

                }
                AnyNode::NestedNode(_) => {
                    let res = format!("{}{{ ▼\n", self.num_spaces((1+self.indent_level)*INDENT_SIZE));
                    results.push(
                        (
                            res,
                            self.node_path.push_and_clone(NodePathKey::DictKey(key.clone()))
                        )
                    );
                    // results.extend(child_node.get_structures())
                    let current_node_owned_formatted_string = format!(
                        "{}\"{}\":",
                        self.num_spaces((self.indent_level+2)*INDENT_SIZE),
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
                    // print closing bracket
                    let closing_bracket_str = format!(
                        "{}}}\n",
                        self.num_spaces((1+self.indent_level)*INDENT_SIZE) 
                    );
                    results.push(
                        (
                            closing_bracket_str,
                            self.node_path.clone(),
                        )
                    );
                } // NOOP 
                AnyNode::PrimitiveNode(_) => {
                    results.extend(child_node.get_structures())
                }
            }


        } 

        // print closing bracket
        let closing_bracket_str = format!(
            "{}]\n",
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


    fn num_spaces(&self, n: u16) -> String {
        " ".repeat(n as usize).to_string()
    }
    pub fn pprint(&self) -> String {
        let mut result = String::new();
        for (str, _) in self.get_structures() { // unoptimised.
            result += &str;
        }
        result
    }
}