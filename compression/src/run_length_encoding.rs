//! Simple implementation of run-length encoding.
//!
//! [`wiki`](https://en.wikipedia.org/wiki/Run-length_encoding)

use std::iter;

use itertools::Itertools;

/// Simple implementation of the run length encoding algorithm.
pub fn run_length_encode(text: &str) -> Vec<(char, i32)> {
    text.chars()
        .map(|c| (c, 1_i32))
        .coalesce(|(x, i), (y, j)| {
            if x == y {
                Ok((x, i + j))
            } else {
                Err(((x, i), (y, j)))
            }
        })
        .collect_vec()
    // .fold(Vec::new(), |mut acc, c| {
    //     match acc.pop() {
    //         None => acc.push(c),
    //         Some((x, i)) if x == c.0 => acc.push((c.0, i + 1)),
    //         Some((x, i)) => {
    //             acc.push((x, i));
    //             acc.push(c);
    //         }
    //     }
    //     acc
    // })
}

/// Simple implementation of the run length decoding algorithm.
pub fn run_length_decode(encoded: &[(char, i32)]) -> String {
    encoded
        .iter()
        .flat_map(|&(c, i)| iter::repeat(c).take(i as usize))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_length_decode() {
        let res = run_length_decode(&[('A', 0)]);
        assert_eq!(res, "");

        let res = run_length_decode(&[('B', 1)]);
        assert_eq!(res, "B");

        let res = run_length_decode(&[('A', 5), ('z', 3), ('B', 1)]);
        assert_eq!(res, "AAAAAzzzB");
    }

    #[test]
    fn test_run_length_encode() {
        assert_eq!(run_length_encode(""), []);
        assert_eq!(run_length_encode("A"), [('A', 1)]);
        assert_eq!(run_length_encode("AA"), [('A', 2)]);
        assert_eq!(run_length_encode("AAAABBBCCDAA"), [
            ('A', 4),
            ('B', 3),
            ('C', 2),
            ('D', 1),
            ('A', 2)
        ]);
        assert_eq!(run_length_encode("Rust-Trends"), [
            ('R', 1),
            ('u', 1),
            ('s', 1),
            ('t', 1),
            ('-', 1),
            ('T', 1),
            ('r', 1),
            ('e', 1),
            ('n', 1),
            ('d', 1),
            ('s', 1)
        ]);
    }
}
