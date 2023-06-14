use std::fmt::Debug;

/// Computes all permutations of an array using Heap's algorithm.
/// Check out [`naive::permute`](crate::permutations::naive) first, since we're
/// building on top of the same intuition
pub fn heap_permute<T: Clone + Debug>(arr: &[T]) -> Vec<Vec<T>> {
    if arr.is_empty() {
        return vec![vec![]];
    }
    let n = arr.len();
    let mut collector = Vec::with_capacity((1..=n).product());
    let mut arr = arr.to_vec();
    heap_recurse(&mut arr, n, &mut collector);
    collector
}

fn heap_recurse<T: Clone + Debug>(arr: &mut [T], k: usize, collector: &mut Vec<Vec<T>>) {
    if k == 1 {
        collector.push((*arr).to_vec());
        return;
    }
    let is_even = k % 2 == 0;
    for i in 0..k {
        arr.swap(if is_even { i } else { 0 }, k - 1);
        heap_recurse(arr, k - 1, collector);
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::heap_permute;
    use crate::permutations::tests::{assert_permutations, assert_valid_permutation, NotTooBigVec};

    #[test]
    fn test_3_different_values() {
        let original = vec![1, 2, 3];
        let res = heap_permute(&original);
        assert_eq!(res.len(), 6); // 3!
        for permute in res {
            assert_valid_permutation(&original, &permute)
        }
    }

    #[test]
    fn test_3_times_the_same_value() {
        let original = vec![1, 1, 1];
        let res = heap_permute(&original);
        assert_eq!(res.len(), 6); // 3!
        for permute in res {
            assert_valid_permutation(&original, &permute)
        }
    }

    #[quickcheck]
    fn test_some_elements(NotTooBigVec { inner: original }: NotTooBigVec) {
        let permutations = heap_permute(&original);
        assert_permutations(&original, &permutations)
    }
}
