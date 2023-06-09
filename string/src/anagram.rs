use itertools::Itertools;

/// Determines if two strings are anagrams.
pub fn check_anagram(s: impl AsRef<str>, t: impl AsRef<str>) -> bool {
    let mut s = s.as_ref().to_ascii_lowercase().chars().collect_vec();
    let mut t = t.as_ref().to_ascii_lowercase().chars().collect_vec();
    s.sort_unstable();
    t.sort_unstable();
    s == t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_anagram() {
        assert!(check_anagram(String::from("anagram"), "nagaram"));
        assert!(!check_anagram("rat", "car"));
        assert!(check_anagram("abcde", "edcba"));
        assert!(check_anagram("sIlEnT", "LiStEn"));
    }
}
