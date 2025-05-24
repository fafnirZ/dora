use std::any::Any;

// parse json into nodes
use serde_json::{Result, Value};

use super::node::Node;

pub fn parse(data: &str) -> Node {
    let v: Value = serde_json::from_str(data).unwrap();
    return Node::new(v).unwrap();
}

// fn create_node(serda_val: Value) -> Node {

// }



#[cfg(test)]
mod tests {
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
        println!("{:?}", n.parse());

        assert!( 1 == 0 );
    }


}