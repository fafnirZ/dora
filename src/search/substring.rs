// if A is substring of B -> [start,end]
// else None
pub fn substring(string_a: &str, string_b: &str) -> Option<[usize; 2]> {
    // we consider empty input to return None
    // the buffer defaults as empty
    // we don't want everything to match when that occurs
    if string_a.is_empty() {
        return None; 
    }
    if string_b.len() < string_a.len() {
        return None;
    }

    let bytes_a = string_a.as_bytes();
    let bytes_b = string_b.as_bytes();
    let len_a = bytes_a.len();

    for i in 0..=(bytes_b.len() - len_a) {
        if &bytes_b[i..i + len_a] == bytes_a {
            return Some([i, i + len_a]);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(substring("aaa", "bbbaaaabb"), Some([3, 6]));
    }

    #[test]
    fn test_b() {
        assert_eq!(substring("xyz", "abc"), None);
    }

    #[test]
    fn test_empty_substring() {
        assert_eq!(substring("", "abc"), Some([0, 0]));
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(substring("abc", ""), None);
    }

    #[test]
    fn test_substring_at_start() {
        assert_eq!(substring("abc", "abcdef"), Some([0, 3]));
    }

    #[test]
    fn test_substring_at_end() {
        assert_eq!(substring("def", "abcdef"), Some([3, 6]));
    }

    #[test]
    fn test_longer_substring() {
        assert_eq!(substring("abcdefg", "abc"), None);
    }
}