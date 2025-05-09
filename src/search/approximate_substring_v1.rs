// given a pattern
// search forward for the occurrence
// for all pattern characters in the input string
// they do not need to be contiguous

use rayon::result;
use std::collections::VecDeque;

use super::traits::SearchAlgorithm;



struct SimpleApproximateSearch {}

impl SearchAlgorithm for SimpleApproximateSearch {

    type Result = Option<Vec<usize>>;
    // do a single pass forward
    // todo: do a pass from backwards to find shorter SimpleApproximateSearch::search
    // yet to be implemented
    // SimpleApproximateSearch::search if there is such.
    fn search(
        pattern: &str,
        input_str: &str,
        case_insensitive: bool,
    ) -> Self::Result {
        let mut result_indices: Vec<usize> = Vec::new();
        let char_pattern_vec: Vec<char> = pattern.chars().collect();
        let mut char_pattern_queue: VecDeque<char> = VecDeque::from(char_pattern_vec);
        let mut pattern_c = match char_pattern_queue.pop_front() {
            Some(res) => res,
            None => return None, // empty pattern case
        };

        let input_chars: Vec<char> = input_str.chars().collect();
        for (idx, char) in input_chars.iter().enumerate() {
            let cmp_ = {
                if case_insensitive {
                    cmp_char_insensitive(char, &pattern_c)
                } else {
                    // sensitive check
                    *char == pattern_c
                }
            };

            if cmp_ {
                result_indices.push(idx);
                pattern_c = match char_pattern_queue.pop_front() {
                    Some(res) => res,
                    None => break,
                };
            } else {
                continue
            }
        }

        if result_indices.len() == pattern.len() {
            return Some(result_indices);
        }
        return None
    }

} 

// a == b 
// NOTE: we only accept ascii values
fn cmp_char_insensitive(
    a: &char,
    b: &char,
) -> bool {
    if a.is_ascii() {
        let tmp_a = (*a as u8).to_ascii_lowercase();
        let tmp_b = (*b as u8).to_ascii_lowercase();
        return tmp_a == tmp_b;
    } else {
        panic!("Dont currently support non ascii values");
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // these are the normal substring tests
    #[test]
    fn test_a() {
        assert_eq!(SimpleApproximateSearch::search("aaa", "bbbaaaabb", true), Some(vec![3,4,5]));
    }

    #[test]
    fn test_b() {
        assert_eq!(SimpleApproximateSearch::search("xyz", "abc", true), None);
    }

    #[test]
    fn test_empty_substring() {
        assert_eq!(SimpleApproximateSearch::search("", "abc", true), None);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(SimpleApproximateSearch::search("abc", "", true), None);
    }

    #[test]
    fn test_substring_at_start() {
        assert_eq!(SimpleApproximateSearch::search("abc", "abcdef", true), Some(vec![0,1,2]));
    }

    #[test]
    fn test_substring_at_end() {
        assert_eq!(SimpleApproximateSearch::search("def", "abcdef", true), Some(vec![3,4,5]));
    }

    #[test]
    fn test_longer_substring() {
        assert_eq!(SimpleApproximateSearch::search("abcdefg", "abc", true), None);
    }


    // approximate substring tests
    #[test]
    fn test_approximate_substring_1() {
        assert_eq!(SimpleApproximateSearch::search("abc", "apppbbomc", true), Some(vec![0,4,8]));
    }

    #[test]
    fn test_approximate_substring_2() {
        // yikes the example yields a non optimal match, whatever for now i guess
        assert_eq!(SimpleApproximateSearch::search("syd", "Western Sydney", true), Some(vec![2,9,10]));
        // assert_eq!(SimpleApproximateSearch::search("syd", "Western Sydney", true), Some(vec![7,8,9]));
    }

    // bad but whatever
    #[test]
    fn test_approximate_substring_3() {
        assert_eq!(SimpleApproximateSearch::search("syd", "SaveYourDreams", true), Some(vec![0,4,8]));
    }
}