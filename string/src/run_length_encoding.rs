/// run length encoding
pub fn run_length_encoding(target: impl AsRef<str>) -> Option<String> {
    let target = target.as_ref();
    if target.trim().is_empty() {
        return None;
    }

    let mut count = 0;
    let (mut base_character, mut encoded_target) = (String::new(), String::new());

    for c in target.chars() {
        let current_str = c.to_string();

        if base_character.is_empty() {
            base_character = current_str.clone();
        }

        if base_character == current_str {
            count += 1;
        } else {
            encoded_target.push_str(&format!("{count}{base_character}"));
            base_character = current_str;
            count = 1;
        }
    }

    Some(format!("{encoded_target}{count}{base_character}"))
}

/// run length decoding
pub fn run_length_decoding(target: impl AsRef<str>) -> Option<String> {
    let target = target.as_ref();
    if target.trim().is_empty() {
        return None;
    }
    target
        .split_inclusive(char::is_alphabetic)
        .map(|seq| {
            let (count, base_character) = seq.split_at(seq.len() - 1);
            count
                .parse::<usize>()
                .ok()
                .map(|count| base_character.repeat(count))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_empty() {
        assert!(run_length_encoding("").is_none())
    }

    #[test]
    fn encode_identical_character() {
        assert_eq!(run_length_encoding("aaaaaaaaaa"), Some("10a".to_string()))
    }
    #[test]
    fn encode_continuous_character() {
        assert_eq!(
            run_length_encoding("abcdefghijk"),
            Some("1a1b1c1d1e1f1g1h1i1j1k".to_string())
        )
    }

    #[test]
    fn encode_random_character() {
        assert_eq!(
            run_length_encoding("aaaaabbbcccccdddddddddd"),
            Some("5a3b5c10d".to_string())
        )
    }

    #[test]
    fn encode_long_character() {
        assert_eq!(
            run_length_encoding(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabbbcccccdddddddddd"
            ),
            Some("200a3b5c10d".to_string())
        )
    }
    #[test]
    fn decode_empty() {
        assert!(run_length_decoding("").is_none())
    }

    #[test]
    fn decode_identical_character() {
        assert_eq!(run_length_decoding("10a"), Some("aaaaaaaaaa".to_string()))
    }
    #[test]
    fn decode_continuous_character() {
        assert_eq!(
            run_length_decoding("1a1b1c1d1e1f1g1h1i1j1k"),
            Some("abcdefghijk".to_string())
        )
    }

    #[test]
    fn decode_random_character() {
        assert_eq!(
            run_length_decoding("5a3b5c10d"),
            Some("aaaaabbbcccccdddddddddd".to_string())
        )
    }

    #[test]
    fn decode_long_character() {
        assert_eq!(
            run_length_decoding(
                "200a3b5c10d"
            ),
            Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabbbcccccdddddddddd".to_string())
        );
        assert_eq!(
            run_length_decoding("300a3b5c100d"),
            Some(
                "a".repeat(300)
                    + "b".repeat(3).as_str()
                    + "c".repeat(5).as_str()
                    + "d".repeat(100).as_str()
            )
        )
    }
}
