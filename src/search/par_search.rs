use rayon::prelude::*;

use super::substring::substring;

// returns vector
// (index_in_input_vec, slice_inside_input_vec_value)
pub fn par_find_substring_matches(
    input: &Vec<String>,
    substring_to_find: &str,
) -> Vec<(usize, [usize;2])> {
    input
        .par_iter()
        .enumerate()
        .filter_map(|(index, s)| {
            substring(substring_to_find, s).map(|match_indices| (index, match_indices))
        })
        .collect()
}