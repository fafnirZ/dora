use super::{approximate_substring_v1::SimpleApproximateSearch, substring::ExactSubstringSearch};

pub trait SearchAlgorithm {
    type Result;
    fn search(
        &self,
        pattern: &str,
        input_str: &str,
        case_insensitive: bool,
    ) -> Option<Self::Result>;
}

// NEED TO DO THIS
// use an enum to wrap all the possible
// search results
// so we can actually match it properly

pub enum AnySearchResult {
    SimpleApproximateSearch(Vec<usize>),
    ExactSubstringSearch([usize; 2]),
}

pub enum SearchAlgorithmImplementations {
    SimpleApproximateSearch(SimpleApproximateSearch),
    ExactSubstringSearch(ExactSubstringSearch),
}
