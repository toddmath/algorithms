//! the Jaro–Winkler distance is a string metric measuring an edit distance
//! between two sequences.
//! It is a variant proposed in 1990 by William E. Winkler
//! of the Jaro distance metric (1989, Matthew A. Jaro).
use std::cmp;

use itertools::Itertools;

pub fn jaro_winkler_distance(a: impl AsRef<str>, b: impl AsRef<str>) -> f64 {
    let (a, b) = (a.as_ref(), b.as_ref());
    let (la, lb) = (a.len(), b.len());
    if la == 0 && lb == 0 {
        return 0.0;
    }
    fn get_matched_characters(a: &str, b: &str) -> String {
        // let (a, b) = (a.as_ref(), b.as_ref());
        let mut b = b.to_string();
        let mut matched = Vec::new();
        // let limit = cmp::min(a.len(), b.len()) / 2;
        let limit = a.len().min(b.len()) / 2;
        for (i, l) in a.chars().enumerate() {
            let left = cmp::max(0, i as i32 - limit as i32) as usize;
            let right = cmp::min(i + limit + 1, b.len());
            if b[left..right].contains(l) {
                matched.push(l);
                let index = b.find(l).expect("this exists");
                let sa = &b[0..index];
                let sb = &b[index + 1..];
                // let (sa, sb) = b.split_once(l).expect("this exists");
                b = format!("{sa} {sb}");
            }
        }
        matched.iter().collect()
    }

    let m1 = get_matched_characters(a, b);
    let m2 = get_matched_characters(b, a);
    let mcount = m1.len();

    let transpositions = m1
        .chars()
        .zip(m2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
        / 2;

    let jaro = if mcount == 0 {
        return 0.0;
    } else {
        (1_f64 / 3_f64)
            * (mcount as f64 / a.len() as f64
                + mcount as f64 / b.len() as f64
                + (mcount - transpositions) as f64 / mcount as f64)
    };

    // let jaro: f64 = {
    //     if mcount == 0 {
    //         return 0.0;
    //     } else {
    //         let count = mcount as f64;
    //         (1f64 / 3f64)
    //             * (count / a.len() as f64 + count / b.len() as f64 + (mcount -
    //               transpositions) as f64 / count)
    //     }
    // };

    let bound = cmp::min(cmp::min(a.len(), b.len()), 4);
    let mut prefix_len = 0.0;
    for (c1, c2) in a[..bound].chars().zip(b[..bound].chars()) {
        if c1 == c2 {
            prefix_len += 1.0;
        } else {
            break;
        }
    }

    jaro + (0.1 * prefix_len * (1.0 - jaro))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jaro_winkler_distance() {
        let a = jaro_winkler_distance("hello", "world");
        assert_eq!(a, 0.4666666666666666);
        let a = jaro_winkler_distance("martha", "marhta");
        assert_eq!(a, 0.9611111111111111);
        let a = jaro_winkler_distance("martha", "marhat");
        assert_eq!(a, 0.9611111111111111);
        let a = jaro_winkler_distance("test", "test");
        assert_eq!(a, 1.0);
        let a = jaro_winkler_distance("test", "");
        assert_eq!(a, 0.0);
        let a = jaro_winkler_distance("hello world", "HeLLo W0rlD");
        assert_eq!(a, 0.6363636363636364);
    }
}
