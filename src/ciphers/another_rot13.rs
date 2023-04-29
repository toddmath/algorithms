//! rot13 or rotate 13 cipher algorithm

pub fn another_rot13<T: AsRef<str>>(text: T) -> String {
    const INPUT: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    const OUTPUT: &str = "NOPQRSTUVWXYZABCDEFGHIJKLMnopqrstuvwxyzabcdefghijklm";
    text.as_ref()
        .chars()
        .map(|c| match INPUT.find(c) {
            Some(i) => OUTPUT.chars().nth(i).unwrap(),
            None => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::another_rot13;

    #[test]
    fn simple() {
        assert_eq!(another_rot13("ABCzyx"), "NOPmlk");
    }

    #[test]
    fn every_alphabetic_character_with_space() {
        assert_eq!(
            another_rot13("The quick brown fox jumps over the lazy dog"),
            "Gur dhvpx oebja sbk whzcf bire gur ynml qbt"
        );
    }

    #[test]
    fn non_ascii() {
        assert_eq!(another_rot13("🎃 Jack-o'-lantern"), "🎃 Wnpx-b'-ynagrea");
    }
}
