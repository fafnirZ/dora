use rayon::prelude::*;

use super::traits::SearchAlgorithm;


// returns vector
// (index_in_input_vec, slice_inside_input_vec_value)
pub fn par_find_substring_matches<T: SearchAlgorithm>(
    algorithm: &T,
    input: &Vec<String>,
    substring_to_find: &str,
) -> Vec<(usize, [usize;2])> {
    let case_insensitive = true;
    input
        .par_iter()
        .enumerate()
        .filter_map(|(index, s)| {
            match algorithm.search(substring_to_find, s, case_insensitive) {
                Some(result) => Some((index, result)),
                None => None,
            }
            // .map(|match_indices| (index, match_indices))
        })
        .collect()
}