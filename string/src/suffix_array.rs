//! In computer science, a suffix array is a sorted array of all suffixes of a
//! string. It is a data structure used in, among others, full-text indices,
//! data-compression algorithms, and the field of bibliometrics. [Source](https://en.wikipedia.org/wiki/Suffix_array)

use std::{cmp::Ordering, mem};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Suffix {
    index: usize,
    rank: (i32, i32),
}

impl Default for Suffix {
    #[inline]
    fn default() -> Self {
        Self::with_rank((-1, -1))
    }
}

impl PartialOrd for Suffix {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Suffix {
    fn cmp(&self, other: &Self) -> Ordering {
        // let ((a1, a2), (b1, b2)) = (self.rank, other.rank);
        match self.rank.0.cmp(&other.rank.0) {
            Ordering::Equal if self.rank.1 < other.rank.1 => Ordering::Less,
            Ordering::Equal => Ordering::Greater,
            ord => ord,
        }
    }
}

impl Suffix {
    #[inline]
    fn new(index: usize, rank: (i32, i32)) -> Self {
        Self { index, rank }
    }

    #[inline]
    fn with_rank(rank: (i32, i32)) -> Self {
        Self { index: 0, rank }
    }
}

/// Generates a suffix array for a given string `txt`.
pub fn generate_suffix_array(txt: impl AsRef<str>) -> Vec<usize> {
    let txt = txt.as_ref();
    let n = txt.len();

    let get_rank =
        |i: usize| (txt.chars().nth(i).expect("this should exists") as u32 - 'a' as u32) as i32;

    let mut suffixes = (0..n)
        .map(|i| {
            Suffix::new(
                i,
                (get_rank(i), if n > (i + 1) { get_rank(i + 1) } else { -1 }),
            )
        })
        .sorted_unstable()
        .collect_vec();
    // suffixes.sort_unstable();

    let mut ind = vec![0; n];
    let mut k = 4;

    while k < 2 * n {
        let mut rank = 0;
        let mut prev_rank_0 = mem::replace(&mut suffixes[0].rank.0, rank);
        ind[suffixes[0].index] = 0;

        for i in 1..n {
            if suffixes[i].rank != (prev_rank_0, suffixes[i - 1].rank.1) {
                rank += 1;
            }
            prev_rank_0 = mem::replace(&mut suffixes[i].rank.0, rank);
            ind[suffixes[i].index] = i;
        }

        for i in 0..n {
            let next_index = suffixes[i].index + (k / 2);
            suffixes[i].rank.1 = if next_index < n {
                suffixes[ind[next_index]].rank.0
            } else {
                -1
            };
        }

        suffixes.sort_unstable();
        k *= 2;
    }

    suffixes.into_iter().map(|suf| suf.index).collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suffix_array() {
        let result = generate_suffix_array("banana");
        assert_eq!(result, vec![5, 3, 1, 0, 4, 2]);
    }
}
