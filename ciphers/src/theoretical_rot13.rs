//! In theory, rot13 only encodes lowercase characters.

/// Theoretical rot13 algorithm. Only encodes lowercase characters!
pub fn theoretical_rot13(text: impl AsRef<str>) -> String {
    text.as_ref()
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                char::from(((c as u8 - b'a' + 13) % 26) + b'a')
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_character() {
        assert_eq!("n", theoretical_rot13("a"));
    }

    #[test]
    fn multiple_characters() {
        assert_eq!("nop op", theoretical_rot13("abc bc"));
    }

    #[test]
    fn non_ascii() {
        assert_eq!("😀ab", theoretical_rot13("😀no"));
    }

    #[test]
    fn encode_twice_decodes() {
        assert_eq!("abcd", theoretical_rot13(theoretical_rot13("abcd")));
    }
}
