/// Generates prime numbers up to `max`.
pub fn prime_numbers(max: usize) -> Vec<usize> {
    let mut result: Vec<usize> = if max >= 2 { vec![2] } else { vec![] };

    result.extend((3..max + 1).step_by(2).filter(|&i| {
        let stop = (i as f64).sqrt() as usize + 1;
        !(3..stop).step_by(2).any(|j| i % j == 0)
    }));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(prime_numbers(0), vec![]);
        assert_eq!(prime_numbers(11), vec![2, 3, 5, 7, 11]);
        assert_eq!(prime_numbers(25), vec![2, 3, 5, 7, 11, 13, 17, 19, 23]);
        assert_eq!(prime_numbers(33), vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31
        ]);
    }
}
