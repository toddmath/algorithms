use std::{collections::HashSet, fmt::Debug, hash::Hash};

/// Here's a basic (naive) implementation for generating permutations
pub fn permute<T: Clone + Debug>(arr: &[T]) -> Vec<Vec<T>> {
    if arr.is_empty() {
        return vec![vec![]];
    }
    fn permute_recurse<T: Clone + Debug>(arr: &mut Vec<T>, k: usize, collector: &mut Vec<Vec<T>>) {
        if k == 1 {
            collector.push(arr.to_vec());
            return;
        }
        for i in 0..k {
            arr.swap(i, k - 1); // swap i with the last character
            permute_recurse(arr, k - 1, collector); // collect the permutations of the rest
            arr.swap(i, k - 1); // swap back to original
        }
    }

    let n = arr.len();
    let mut collector = Vec::with_capacity((1..=n).product());
    let mut arr = arr.to_vec();

    permute_recurse(&mut arr, n, &mut collector);
    collector
}

/// A common variation of generating permutations is to generate only unique
/// permutations Of course, we could use the version above together with a Set
/// as collector instead of a Vec. But let's try something different: how can we
/// avoid to generate duplicated permutations in the first place, can we tweak
/// the algorithm above?
pub fn permute_unique<T>(arr: &[T]) -> Vec<Vec<T>>
where
    T: Copy + Debug + Eq + Hash,
{
    if arr.is_empty() {
        return vec![vec![]];
    }
    fn permute_recurse_unique<T>(arr: &mut Vec<T>, k: usize, collector: &mut Vec<Vec<T>>)
    where
        T: Copy + Debug + Eq + Hash,
    {
        if k == 1 {
            collector.push(arr.to_vec());
            return;
        }
        let mut swapped = HashSet::with_capacity(k);
        for i in 0..k {
            // if swapped.contains(&arr[i]) {
            //     continue;
            // }
            if !swapped.insert(arr[i]) {
                continue;
            }
            arr.swap(i, k - 1);
            permute_recurse_unique(arr, k - 1, collector);
            arr.swap(i, k - 1);
        }
    }

    let n = arr.len();
    let mut collector = Vec::with_capacity((1..=n).product());
    let mut arr = arr.to_vec();

    permute_recurse_unique(&mut arr, n, &mut collector);
    collector
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use quickcheck_macros::quickcheck;

    use super::*;
    use crate::permutations::tests::{assert_permutations, assert_valid_permutation, NotTooBigVec};

    #[test]
    fn test_3_different_values() {
        let original = vec![1, 2, 3];
        let res = permute(&original);
        assert_eq!(res.len(), 6); // 3!
        for permute in res {
            assert_valid_permutation(&original, &permute)
        }
    }

    #[test]
    fn test_3_times_the_same_value() {
        let original = vec![1, 1, 1];
        let res = permute(&original);
        assert_eq!(res.len(), 6); // 3!
        for permute in res {
            assert_valid_permutation(&original, &permute)
        }
    }

    #[quickcheck]
    fn test_some_elements(NotTooBigVec { inner: original }: NotTooBigVec) {
        let permutations = permute(&original);
        assert_permutations(&original, &permutations)
    }

    #[test]
    fn test_unique_values() {
        let original = vec![1, 1, 2, 2];
        let unique_permutations = permute_unique(&original);
        let every_permutation = permute(&original);
        for unique_permutation in &unique_permutations {
            assert!(every_permutation.contains(unique_permutation));
        }
        assert_eq!(
            unique_permutations.len(),
            every_permutation.iter().collect::<HashSet<_>>().len()
        )
    }
}
