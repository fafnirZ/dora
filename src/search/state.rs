use super::{approximate_substring_v1::SimpleApproximateSearch, traits::{AnySearchResult, SearchAlgorithmImplementations}};

pub struct SearchResultState {
    // assuming you're only performing on a single column
    // the current focused column;
    pub search_algorithm: SearchAlgorithmImplementations,
    pub result_indices: Vec<(usize, AnySearchResult)>,
    pub result_cursor: Option<usize>,
}

impl SearchResultState {
    pub fn new() -> Self {
        Self {
            search_algorithm: SearchAlgorithmImplementations::SimpleApproximateSearch(SimpleApproximateSearch{}),
            result_indices: Vec::new(),
            result_cursor: None,
        }
    }
}
