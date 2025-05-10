
pub trait SearchAlgorithm {
    type Result;
    fn search(
        &self,
        pattern: &str, 
        input_str: &str, 
        case_insensitive: bool
    ) -> Option<Self::Result>;
}


// NEED TO DO THIS
// use an enum to wrap all the possible
// search results

pub enum AnySearchResult {
    SimpleApproximateSearch(Vec<usize>),
    ExactSubstringSearch([usize;2]),
}