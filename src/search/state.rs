
pub struct SearchResultState {
    // assuming you're only performing on a single column
    // the current focused column;
    pub result_indices: Vec<(usize, [usize;2])>
}

impl SearchResultState {
    pub fn new() -> Self {
        Self {
            result_indices: Vec::new(),
        }
    }
}