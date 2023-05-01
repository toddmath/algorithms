use super::{is_mersenne_prime, prime_numbers};

/// Generates a list of perfect numbers till `num` using the
/// [`Lucas Lehmer test`](https://en.wikipedia.org/wiki/Lucas%E2%80%93Lehmer_primality_test) algorithm.
pub fn generate_perfect_numbers(num: usize) -> Vec<usize> {
    prime_numbers((((num * 8 + 1) as f64).log2() as usize) / 2)
        .into_iter()
        .filter_map(|n| {
            if is_mersenne_prime(n) {
                Some(
                    (2usize.pow(u32::try_from(n).unwrap_or_default()) - 1)
                        * (2usize.pow(u32::try_from(n - 1).unwrap_or_default())),
                )
            } else {
                None
            }
        })
        .filter(|&n| n <= num)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perfect_numbers_till_n() {
        let n = 335564540;
        assert_eq!(generate_perfect_numbers(n), [6, 28, 496, 8128, 33550336]);
        assert_eq!(generate_perfect_numbers(40), [6, 28]);
        assert_eq!(generate_perfect_numbers(0), []);
    }
}
