use unicode_segmentation::UnicodeSegmentation;

pub fn dual_iterator_palindrome(word: &str) -> bool {
    word.graphemes(true).eq(word.graphemes(true).rev())
}

pub fn simple_palindrome(word: &str) -> bool {
    let mut chars = word.graphemes(true);

    loop {
        let s = chars.next();
        let e = chars.next_back();
        match (s, e) {
            (Some(s), Some(e)) => {
                if s != e {
                    return false;
                }
            }
            (Some(s), None) => {
                // even string and we're passed the middle
                return true;
            }
            (None, Some(e)) => {
                // odd string and we're at the middle
                return true;
            }
            (None, None) => {
                // an empty string
                return true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_simple_empty() {
        assert_eq!(simple_palindrome(""), true);
        assert_eq!(dual_iterator_palindrome(""), true);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(simple_palindrome("a"), true);
        assert_eq!(dual_iterator_palindrome("a"), true);
    }

    #[test]
    fn test_mutli_char() {
        assert_eq!(simple_palindrome("aaaaa"), true);
        assert_eq!(dual_iterator_palindrome("aaaaa"), true);
    }

    #[test]
    fn test_mutli_char_obvious_false() {
        assert_eq!(simple_palindrome("abcdef"), false);
        assert_eq!(dual_iterator_palindrome("abcdef"), false);
    }

    #[test]
    fn test_mutli_different_true() {
        assert_eq!(simple_palindrome("aba"), true);
        assert_eq!(dual_iterator_palindrome("aba"), true);
    }
}
