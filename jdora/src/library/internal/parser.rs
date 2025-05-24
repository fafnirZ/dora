use std::any::Any;

// parse json into nodes
use serde_json::{Result, Value};

pub fn parse(data: &str) {
    let v: Value = serde_json::from_str(data).unwrap();

    println!("{}", v.keys());
}

fn create_node(serda_val: Value) -> Node {

}



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
        parse(data);
        assert!( 1 == 0 );
    }


}