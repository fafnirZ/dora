
pub trait SearchAlgorithm {
    type Result;
    fn search(
        pattern: &str, 
        input_str: &str, 
        case_insensitive: bool
    ) -> Self::Result;
}