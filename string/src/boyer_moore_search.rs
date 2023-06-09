use std::{cmp, collections::HashMap};

use itertools::Itertools;

/// In computer science, the Boyer–Moore string-search algorithm is an efficient
/// string-searching algorithm, that is the standard benchmark for practical
/// string-search literature. [Wikipedia](https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_string-search_algorithm)
pub fn boyer_moore_search(text: impl AsRef<str>, pattern: impl AsRef<str>) -> Vec<usize> {
    let (text, pattern) = (text.as_ref(), pattern.as_ref());
    let mut positions = Vec::new();
    let (n, m) = (text.len() as i32, pattern.len() as i32);
    if n == 0 || m == 0 {
        return positions;
    }

    let pattern = pattern.chars().collect_vec();
    let text = text.chars().collect_vec();
    let collection: HashMap<&char, i32> = pattern
        .iter()
        .enumerate()
        .map(|(i, c)| (c, i as i32))
        .collect();

    let mut shift = 0i32;

    while shift <= (n - m) {
        let mut j = m - 1;
        while j >= 0 && pattern[j as usize] == text[(shift + j) as usize] {
            j -= 1;
        }

        if j < 0 {
            positions.push(shift as usize);
            shift += if (shift + m) < n {
                m - collection.get(&text[(shift + m) as usize]).unwrap_or(&-1)
            } else {
                1
            };
        } else {
            shift += cmp::max(
                1,
                j - collection.get(&text[(shift + j) as usize]).unwrap_or(&-1),
            );
        }
    }
    positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boyer_moore_search() {
        let a = boyer_moore_search("AABCAB12AFAABCABFFEGABCAB", "ABCAB");
        assert_eq!(a, [1, 11, 20]);

        let a = boyer_moore_search("AABCAB12AFAABCABFFEGABCAB", "FFF");
        assert_eq!(a, []);

        let a = boyer_moore_search("AABCAB12AFAABCABFFEGABCAB", "CAB");
        assert_eq!(a, [3, 13, 22]);
    }
}
