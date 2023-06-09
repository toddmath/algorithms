//! Solves the rod-cutting problem

use num_traits::PrimInt;

/// [`rod_cut`] returns the maximum possible profit if a rod of length `n` =
/// `p.len()` is cut into up to `n` pieces, where the profit gained from each
/// piece of length `l` is determined by `p[l - 1]` and the total profit is the
/// sum of the profit gained from each piece.
///
/// # Arguments
///    - `p` - profit for rods of length 1 to n inclusive
///
/// # Complexity
///    - time complexity: O(n^2),
///    - space complexity: O(n^2),
///
/// where n is the length of `p`.
pub fn rod_cut<T: PrimInt>(p: &[T]) -> T {
    if p.is_empty() {
        return T::zero();
    }
    let n = p.len();
    let mut f = vec![T::zero(); n];

    for i in 0..n {
        f[i] = (1..=i)
            .map(|j| p[j - 1] + f[i - j])
            .max()
            .unwrap_or(p[i])
            .max(p[i]);
    }

    f[n - 1]
}

#[cfg(test)]
mod tests {
    use super::rod_cut;

    #[test]
    fn test_rod_cut() {
        assert_eq!(0, rod_cut(&[]));
        assert_eq!(15, rod_cut(&[5, 8, 2]));
        assert_eq!(10, rod_cut(&[1, 5, 8, 9]));
        assert_eq!(25, rod_cut(&[5, 8, 2, 1, 7]));
        assert_eq!(87, rod_cut(&[0, 0, 0, 0, 0, 87]));
        assert_eq!(49, rod_cut(&[7, 6, 5, 4, 3, 2, 1]));
        assert_eq!(22, rod_cut(&[1, 5, 8, 9, 10, 17, 17, 20]));
        assert_eq!(60, rod_cut(&[6, 4, 8, 2, 5, 8, 2, 3, 7, 11]));
        assert_eq!(30, rod_cut(&[1, 5, 8, 9, 10, 17, 17, 20, 24, 30]));
        assert_eq!(12, rod_cut(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]));
    }
}
