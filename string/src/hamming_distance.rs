/// Hamming Distance
///
/// # Panics
///
/// Panics if the strings are not the same length.
pub fn hamming_distance(s1: impl AsRef<str>, s2: impl AsRef<str>) -> usize {
    let (mut s1, mut s2) = (s1.as_ref().chars(), s2.as_ref().chars());
    let mut dist = 0;

    loop {
        match (s1.next(), s2.next()) {
            (Some(c1), Some(c2)) if c1 != c2 => dist += 1,
            (Some(_), Some(_)) => continue,
            (None, Some(_)) | (Some(_), None) => panic!("Strings must have the same length"),
            (None, None) => break,
        }
    }

    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_strings() {
        let result = hamming_distance("", "");
        assert_eq!(result, 0);
    }

    #[test]
    fn distance_zero() {
        let result = hamming_distance("rust", "rust");
        assert_eq!(result, 0);
    }

    #[test]
    fn distance_three() {
        let result = hamming_distance("karolin", "kathrin");
        assert_eq!(result, 3);
    }

    #[test]
    fn distance_four() {
        let result = hamming_distance("kathrin", "kerstin");
        assert_eq!(result, 4);
    }

    #[test]
    fn distance_five() {
        let result = hamming_distance("00000", "11111");
        assert_eq!(result, 5);
    }
}
