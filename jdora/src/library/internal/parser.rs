// parse json into nodes

pub fn parse(data: &str) {

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
        assert_eq!(1==0);
    }


}