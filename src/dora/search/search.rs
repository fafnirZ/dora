use super::traits::SearchAlgorithm;
use rayon::prelude::*;

// returns vector of tuples:
// (index in input vec, where the string is matched, Matching function's results)
pub fn par_find_substring_matches<T: SearchAlgorithm + Send + Sync>(
    algorithm: &T,
    input: &Vec<String>,
    substring_to_find: &str,
) -> Vec<(usize, T::Result)>
where
    T::Result: Send,
{
    let case_insensitive = true;
    input
        .par_iter()
        .enumerate()
        .filter_map(
            |(index, s)| match algorithm.search(substring_to_find, s, case_insensitive) {
                Some(result) => Some((index, result)),
                None => None,
            },
        )
        .collect()
}
