/// Determines if `a` is a subsequence of `b`.
///
/// A subsequence of a string is a new string that is formed from the original
/// string by deleting some (can be none) of the characters without disturbing
/// the relative positions of the remaining characters. (i.e., `"ace"` is a
/// subsequence of `"abcde"` while `"aec"` is not).
#[inline]
pub fn is_subsequence<T: AsRef<str>>(a: T, b: T) -> bool {
    a.as_ref()
        .chars()
        .all(|c| b.as_ref().chars().any(|x| x == c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(is_subsequence("abc", "ahbgdc"));
        assert!(!is_subsequence("axc", "ahbgdc"));
    }
}
