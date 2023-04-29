/// [`mersenne_prime`](https://en.wikipedia.org/wiki/Mersenne_prime)
pub fn is_mersenne_prime(n: usize) -> bool {
    if n == 2 {
        return true;
    }
    let mut s = 4;
    let m = 2usize.pow(TryInto::try_into(n).unwrap()) - 1;
    for _ in 0..n - 2 {
        s = ((s * s) - 2) % m;
    }
    s == 0
}

/// [`mersenne_prime`](https://en.wikipedia.org/wiki/Mersenne_prime)
pub fn get_mersenne_primes(limit: usize) -> Vec<usize> {
    (1..=limit).filter(|&n| is_mersenne_prime(n)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validity_check() {
        assert!(is_mersenne_prime(3));
        assert!(is_mersenne_prime(13));
        assert!(!is_mersenne_prime(32));
    }

    #[allow(dead_code)]
    fn generation_check() {
        assert_eq!(get_mersenne_primes(30), [2, 3, 5, 7, 13, 17, 19]);
    }
}
