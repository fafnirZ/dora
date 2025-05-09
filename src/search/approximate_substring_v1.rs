// given a pattern
// search forward for the occurrence
// for all pattern characters in the input string
// they do not need to be contiguous

use rayon::result;
use std::collections::VecDeque;

// do a single pass forward
// todo: do a pass from backwards to find shorter approx_substring
// yet to be implemented
// approx_substring if there is such.
pub fn approx_substring(
    pattern: &str,
    input_str: &str,
) -> Option<Vec<usize>> {
    let mut result_indices: Vec<usize> = Vec::new();
    let char_pattern_vec: Vec<char> = pattern.chars().collect();
    let mut char_pattern_queue: VecDeque<char> = VecDeque::from(char_pattern_vec);
    let mut pattern_c = match char_pattern_queue.pop_front() {
        Some(res) => res,
        None => return None, // empty pattern case
    };

    let input_chars: Vec<char> = input_str.chars().collect();
    for (idx, char) in input_chars.iter().enumerate() {
        if *char == pattern_c {
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



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // these are the normal substring tests
    #[test]
    fn test_a() {
        assert_eq!(approx_substring("aaa", "bbbaaaabb"), Some(vec![3,4,5]));
    }

    #[test]
    fn test_b() {
        assert_eq!(approx_substring("xyz", "abc"), None);
    }

    #[test]
    fn test_empty_substring() {
        assert_eq!(approx_substring("", "abc"), None);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(approx_substring("abc", ""), None);
    }

    #[test]
    fn test_substring_at_start() {
        assert_eq!(approx_substring("abc", "abcdef"), Some(vec![0,1,2]));
    }

    #[test]
    fn test_substring_at_end() {
        assert_eq!(approx_substring("def", "abcdef"), Some(vec![3,4,5]));
    }

    #[test]
    fn test_longer_substring() {
        assert_eq!(approx_substring("abcdefg", "abc"), None);
    }


    // approximate substring tests

}