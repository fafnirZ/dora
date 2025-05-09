use super::traits::SearchAlgorithm;

struct ExactSubstringSearch {}

impl SearchAlgorithm for ExactSubstringSearch {
    type Result = [usize;2];
    // if A is substring of B -> [start,end]
    // pattern is the substring we wish to search for
    // input_str is the target string we with to check for substring containment
    // else None
    fn search(&self, pattern: &str, input_str: &str, _case_insensitive: bool) -> Option<Self::Result> {
        // we consider empty input to return None
        // the buffer defaults as empty
        // we don't want everything to match when that occurs
        if pattern.is_empty() {
            return None; 
        }
        if input_str.len() < pattern.len() {
            return None;
        }

        let bytes_a = pattern.as_bytes();
        let bytes_b = input_str.as_bytes();
        let len_a = bytes_a.len();

        for i in 0..=(bytes_b.len() - len_a) {
            if &bytes_b[i..i + len_a] == bytes_a {
                return Some([i, i + len_a]);
            }
        }
        None
    }

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(ExactSubstringSearch{}.search("aaa", "bbbaaaabb", true), Some([3, 6]));
    }

    #[test]
    fn test_b() {
        assert_eq!(ExactSubstringSearch{}.search("xyz", "abc", true), None);
    }

    #[test]
    fn test_empty_substring() {
        assert_eq!(ExactSubstringSearch{}.search("", "abc", true), None);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(ExactSubstringSearch{}.search("abc", "", true), None);
    }

    #[test]
    fn test_substring_at_start() {
        assert_eq!(ExactSubstringSearch{}.search("abc", "abcdef", true), Some([0, 3]));
    }

    #[test]
    fn test_substring_at_end() {
        assert_eq!(ExactSubstringSearch{}.search("def", "abcdef", true), Some([3, 6]));
    }

    #[test]
    fn test_longer_substring() {
        assert_eq!(ExactSubstringSearch{}.search("abcdefg", "abc", true), None);
    }
}