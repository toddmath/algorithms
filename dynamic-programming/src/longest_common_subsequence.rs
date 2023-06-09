//! Longest common subsequence via Dynamic Programming
use std::cmp;

use itertools::Itertools;

/// Returns the longest common subsequence between `a` and `b`.
pub fn longest_common_subsequence(a: impl AsRef<str>, b: impl AsRef<str>) -> String {
    let (a, b) = (a.as_ref(), b.as_ref());
    let a = a.chars().collect_vec();
    let b = b.chars().collect_vec();
    let (mut na, mut nb) = (a.len(), b.len());

    let mut solutions = vec![vec![0; nb + 1]; na + 1];

    for ((i, ci), (j, cj)) in a.iter().enumerate().cartesian_product(b.iter().enumerate()) {
        solutions[i + 1][j + 1] = if ci == cj {
            solutions[i][j] + 1
        } else {
            cmp::max(solutions[i][j + 1], solutions[i + 1][j])
        };
    }

    let mut result = String::with_capacity(cmp::max(na, nb));

    while na > 0 && nb > 0 {
        if a[na - 1] == b[nb - 1] {
            result.push(a[na - 1]);
            na -= 1;
            nb -= 1;
            continue;
        }
        if solutions[na - 1][nb] > solutions[na][nb - 1] {
            na -= 1;
        } else {
            nb -= 1;
        }
    }

    result.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::longest_common_subsequence;

    #[test]
    fn test_longest_common_subsequence() {
        // empty case
        assert_eq!(&longest_common_subsequence("", ""), "");
        assert_eq!(&longest_common_subsequence("", "abcd"), "");
        assert_eq!(&longest_common_subsequence("abcd", ""), "");

        // simple cases
        assert_eq!(&longest_common_subsequence("abcd", "c"), "c");
        assert_eq!(&longest_common_subsequence("abcd", "d"), "d");
        assert_eq!(&longest_common_subsequence("abcd", "e"), "");
        assert_eq!(&longest_common_subsequence("abcdefghi", "acegi"), "acegi");

        // less simple cases
        assert_eq!(&longest_common_subsequence("abcdgh", "aedfhr"), "adh");
        assert_eq!(&longest_common_subsequence("aggtab", "gxtxayb"), "gtab");

        // unicode
        assert_eq!(
            &longest_common_subsequence("你好，世界", "再见世界"),
            "世界"
        );
    }
}
