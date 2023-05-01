/// Collatz sequence
pub fn sequence(mut n: usize) -> Option<Vec<usize>> {
    if n == 0 {
        return None;
    }
    let mut list = Vec::with_capacity(n);
    while n != 1 {
        list.push(n);
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
    }
    list.push(n);
    Some(list)
}

#[cfg(test)]
mod tests {
    use super::sequence;

    #[test]
    fn is_valid() {
        assert_eq!(sequence(10).unwrap(), [10, 5, 16, 8, 4, 2, 1]);
        assert_eq!(
            sequence(15).unwrap(),
            [15, 46, 23, 70, 35, 106, 53, 160, 80, 40, 20, 10, 5, 16, 8, 4, 2, 1]
        );
        assert!(sequence(0).is_none());
    }
}
