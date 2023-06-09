const MODULUS: u16 = 101;
const BASE: u16 = 256;

/// Rabin Karp algorithm
pub fn rabin_karp(target: impl AsRef<str>, pattern: impl AsRef<str>) -> Vec<usize> {
    let (target, pattern) = (target.as_ref(), pattern.as_ref());

    if target.is_empty() || pattern.is_empty() || pattern.len() > target.len() {
        return vec![];
    }

    let pattern_hash = hash(pattern);
    let pow_rem = (0..pattern.len() - 1).fold(1u16, |acc, _| (acc * BASE) % MODULUS);

    let mut rolling_hash = 0;
    let mut ret = Vec::with_capacity(target.len() - pattern.len() + 1);

    for i in 0..=target.len() - pattern.len() {
        rolling_hash = if i == 0 {
            hash(target[0..pattern.len()].as_ref())
        } else {
            recalculate_hash(target, i - 1, i + pattern.len() - 1, rolling_hash, pow_rem)
        };

        if rolling_hash == pattern_hash && pattern == &target[i..pattern.len() + i] {
            ret.push(i);
        }
    }
    ret
}

#[inline]
fn hash(s: &str) -> u16 {
    s.as_bytes()
        .iter()
        .fold(0_u16, |acc, c| (acc * BASE % MODULUS + *c as u16) % MODULUS)
}

fn recalculate_hash(
    s: &str,
    old_index: usize,
    new_index: usize,
    old_hash: u16,
    pow_rem: u16,
) -> u16 {
    let mut new_hash = old_hash;
    let bytes = s.as_bytes();
    new_hash += (MODULUS - pow_rem * bytes[old_index] as u16 % MODULUS) % MODULUS;
    (new_hash * BASE % MODULUS + bytes[new_index] as u16) % MODULUS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hi_hash() {
        let hash_result = hash("hi");
        assert_eq!(hash_result, 65);
    }

    #[test]
    fn abr_hash() {
        let hash_result = hash("abr");
        assert_eq!(hash_result, 4);
    }

    #[test]
    fn bra_hash() {
        let hash_result = hash("bra");
        assert_eq!(hash_result, 30);
    }

    // Attribution to @pgimalac for his tests from Knuth-Morris-Pratt
    #[test]
    fn each_letter_matches() {
        let index = rabin_karp("aaa", "a");
        assert_eq!(index, vec![0, 1, 2]);
    }

    #[test]
    fn a_few_separate_matches() {
        let index = rabin_karp("abababa", "ab");
        assert_eq!(index, vec![0, 2, 4]);
    }

    #[test]
    fn one_match() {
        let index = rabin_karp("ABC ABCDAB ABCDABCDABDE", "ABCDABD");
        assert_eq!(index, vec![15]);
    }

    #[test]
    fn lots_of_matches() {
        let index = rabin_karp("aaabaabaaaaa", "aa");
        assert_eq!(index, vec![0, 1, 4, 7, 8, 9, 10]);
    }

    #[test]
    fn lots_of_intricate_matches() {
        let index = rabin_karp("ababababa", "aba");
        assert_eq!(index, vec![0, 2, 4, 6]);
    }

    #[test]
    fn not_found0() {
        let index = rabin_karp("abcde", "f");
        assert_eq!(index, vec![]);
    }

    #[test]
    fn not_found1() {
        let index = rabin_karp("abcde", "ac");
        assert_eq!(index, vec![]);
    }

    #[test]
    fn not_found2() {
        let index = rabin_karp("ababab", "bababa");
        assert_eq!(index, vec![]);
    }

    #[test]
    fn empty_string() {
        let index = rabin_karp("", "abcdef");
        assert_eq!(index, vec![]);
    }
}
