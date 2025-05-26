use std::any::Any;

// parse json into nodes
use serde_json::{Result, Value};

use super::{node::Node, node_path::NodePath};

pub fn parse(data: &str) -> Node {
    let v: Value = serde_json::from_str(data).unwrap();
    return Node::new(v, NodePath::new()).unwrap();
}
pub fn parse_bytes(data: &[u8]) -> Node {
    let v: Value = serde_json::from_slice(data).unwrap();
    return Node::new(v, NodePath::new()).unwrap();
}



#[cfg(test)]
mod tests {
    use crate::library::internal::node_path::NodePath;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_a() {
        let data = r#"
        {
            "name": "abc",
            "hello": 1,
            "nested": {
                "attr": 2
            }
        }"#;
        let n = parse(data);
        println!("{:?}", n.get_structures());

        assert!( 1 == 0 );
    }

    #[test]
    fn test_b() {
        let data = r#"
        {
            "name": "abc",
            "hello": 1,
            "nested": {
                "attr": 2
            }
        }"#;
        let mut n = parse(data);
        n.hidden_children.push(0);
        println!("{:?}", n.get_structures());

        assert!( 1 == 0 );
    }
    // #[test]
    // fn test_c() {
    //     let data = r#"
    //     {
    //         "name": "abc",
    //         "hello": 1,
    //         "nested": {
    //             "attr": 2
    //         }
    //     }"#;
    //     let mut n = parse(data);
    //     let node_path = NodePath::new();
    //     println!("{:?}", n.build_children_line_boundaries(0, &node_path));

    //     assert!( 1 == 0 );
    // }



    // #[test]
    // fn test_d() {
    //     let data = r#"
    //     {
    //         "name": "abc",
    //         "hello": 1,
    //         "nested": {
    //             "attr": {
    //                 "bbb": 100
    //             }
    //         }
    //     }"#;
    //     let mut n = parse(data);
    //     let node_path = NodePath::new();
    //     println!("{:?}", n.build_children_line_boundaries(0, &node_path));

    //     assert!( 1 == 0 );
    // }

    // #[test]
    // fn test_e() {
    //     let data = r#"
    //     {
    //         "name": "abc",
    //         "hello": 1,
    //         "nested": {
    //             "attr": {
    //                 "bbb": 100
    //             }
    //         },
    //         "another_nested": {
    //             "b": "bbb"
    //         }
    //     }"#;
    //     let mut n = parse(data);
    //     let node_path = NodePath::new();
    //     println!("{:?}", n.build_children_line_boundaries(0, &node_path));

    //     assert!( 1 == 0 );
    // }
}