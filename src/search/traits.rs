
pub trait SearchAlgorithm {
    type Result;
    fn search(
        &self,
        pattern: &str, 
        input_str: &str, 
        case_insensitive: bool
    ) -> Option<Self::Result>;
}