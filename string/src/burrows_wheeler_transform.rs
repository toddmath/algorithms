use itertools::Itertools;

/// Burrows Wheeler Transform
pub fn burrows_wheeler_transform(input: impl AsRef<str>) -> (String, usize) {
    let input = input.as_ref();
    let len = input.len();
    let table = (0..len)
        .map(|i| format!("{}{}", &input[i..], &input[..i]))
        .sorted_unstable_by_key(|a| a.to_lowercase())
        .collect_vec();

    let mut encoded = String::with_capacity(len);
    let mut index = 0;
    for (i, item) in table.iter().enumerate().take(len) {
        encoded.push(item.chars().last().unwrap());
        if item == input {
            index = i;
        }
    }

    (encoded, index)
}

/// Inverse Burrows Wheeler Transform
pub fn inv_burrows_wheeler_transform<T: AsRef<str>>((input, index): (T, usize)) -> String {
    let input = input.as_ref();
    let len = input.len();
    let table = input
        .char_indices()
        .sorted_unstable_by_key(|a| a.1)
        .collect_vec();

    let mut decoded = String::with_capacity(len);
    let mut index = index;
    for _ in 0..len {
        decoded.push(table[index].1);
        index = table[index].0;
    }

    decoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Ensure function stand-alone legitimacy
    fn stand_alone_function() {
        assert_eq!(
            burrows_wheeler_transform("CARROT"),
            ("CTRRAO".to_string(), 1usize)
        );
        assert_eq!(
            inv_burrows_wheeler_transform(("CTRRAO".to_string(), 1usize)),
            ("CARROT".to_string())
        );
        assert_eq!(
            burrows_wheeler_transform("THEALGORITHMS"),
            ("EHLTTRAHGOMSI".to_string(), 11usize)
        );
        assert_eq!(
            inv_burrows_wheeler_transform(("EHLTTRAHGOMSI".to_string(), 11usize)),
            ("THEALGORITHMS".to_string())
        );
        assert_eq!(
            burrows_wheeler_transform("!.!.!??.=::"),
            (":..!!?:=.?!".to_string(), 0usize)
        );
        assert_eq!(
            inv_burrows_wheeler_transform((":..!!?:=.?!".to_string(), 0usize)),
            "!.!.!??.=::"
        );
    }

    #[test]
    fn basic_characters() {
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("CARROT")),
            "CARROT"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("TOMATO")),
            "TOMATO"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("THISISATEST")),
            "THISISATEST"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("THEALGORITHMS")),
            "THEALGORITHMS"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("RUST")),
            "RUST"
        );
    }

    #[test]
    fn special_characters() {
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("!.!.!??.=::")),
            "!.!.!??.=::"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("!{}{}(((&&%%!??.=::")),
            "!{}{}(((&&%%!??.=::"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("//&$[]")),
            "//&$[]"
        );
    }

    #[test]
    fn empty() {
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("")),
            ""
        );
    }
}
