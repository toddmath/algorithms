//! rot13 or rotate 13 cipher algorithm

/// Rotate 13 cipher algorithm.
pub fn rot13<T: AsRef<str>>(text: T) -> String {
    text.as_ref()
        .to_uppercase()
        .chars()
        .map(|c| match c {
            'A'..='M' => ((c as u8) + 13) as char,
            'N'..='Z' => ((c as u8) - 13) as char,
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::rot13;

    #[test]
    fn single_character() {
        assert_eq!(rot13("A"), "N");
    }

    #[test]
    fn multiple_characters() {
        assert_eq!(rot13("ABC"), "NOP");
    }

    #[test]
    fn non_ascii() {
        assert_eq!(rot13("😀AB"), "😀NO");
    }

    #[test]
    fn double_encoded() {
        assert_eq!(rot13(rot13("ABCD")), "ABCD");
    }
}
