/// Reverse a string
#[inline]
pub fn reverse(text: impl AsRef<str>) -> String {
    text.as_ref().chars().rev().collect()
}

// cSpell: disable
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(reverse("racecar"), "racecar");
    }

    #[test]
    fn test_asymmetric() {
        assert_eq!(reverse("abcdef"), "fedcba")
    }

    #[test]
    fn test_sentence() {
        assert_eq!(reverse("step on no pets"), "step on no pets");
    }
}
