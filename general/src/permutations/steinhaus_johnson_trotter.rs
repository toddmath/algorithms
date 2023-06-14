use super::Permuted;

/// Steinhaus Johnson Trotter permute.
/// [Wikipedia Reference](https://en.wikipedia.org/wiki/Steinhaus%E2%80%93Johnson%E2%80%93Trotter_algorithm)
pub fn steinhaus_johnson_trotter_permute<T: Clone>(arr: &[T]) -> Permuted<T> {
    let (n, mut i) = (arr.len(), 1);
    let (mut arr, mut inversion_vector) = (arr.to_vec(), vec![0; n]);
    let mut res = Vec::with_capacity((1..=n).product());
    res.push(arr.clone());

    while i < n {
        if inversion_vector[i] < i {
            arr.swap(if i % 2 == 0 { 0 } else { inversion_vector[i] }, i);
            res.push(arr.to_vec());
            inversion_vector[i] += 1;
            i = 1;
        } else {
            inversion_vector[i] = 0;
            i += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::*;
    use crate::permutations::tests::{assert_permutations, assert_valid_permutation, NotTooBigVec};

    #[test]
    fn test_3_different_values() {
        let original = vec![1, 2, 3];
        let res = steinhaus_johnson_trotter_permute(&original);
        assert_eq!(res.len(), 6); // 3!
        for permute in res {
            assert_valid_permutation(&original, &permute)
        }
    }

    #[test]
    fn test_3_times_the_same_value() {
        let original = vec![1, 1, 1];
        let res = steinhaus_johnson_trotter_permute(&original);
        assert_eq!(res.len(), 6); // 3!
        for permute in res {
            assert_valid_permutation(&original, &permute)
        }
    }

    #[quickcheck]
    fn test_some_elements(NotTooBigVec { inner: original }: NotTooBigVec) {
        let permutations = steinhaus_johnson_trotter_permute(&original);
        assert_permutations(&original, &permutations)
    }
}
