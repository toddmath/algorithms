/// Deterimines if a value is a valid palindrome.
pub fn is_palindrome<T: AsRef<str>>(s: T) -> bool {
    let mut chars = s.as_ref().chars();
    while let (Some(head), Some(tail)) = (chars.next(), chars.next_back()) {
        if head != tail {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palindromes() {
        assert!(is_palindrome("abcba"));
        assert!(is_palindrome("abba"));
        assert!(is_palindrome("a"));
        assert!(is_palindrome("arcra"));
        assert!(!is_palindrome("abcde"));
        assert!(!is_palindrome("aaaabbbb"));
    }
}
