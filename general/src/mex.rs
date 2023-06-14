use std::collections::BTreeSet;

use itertools::Itertools;
// use itertools::Itertools;
use num_traits::{PrimInt, Signed};

// Find minimum excluded number from a set of given numbers using a set
/// Finds the MEX of the values provided in `arr` using a
/// [`BTreeSet`](std::collections::BTreeSet).
///
/// `O(n-log(n))` implementation
pub fn mex_using_set<T: PrimInt + Signed>(arr: &[T]) -> T {
    let mut s = BTreeSet::new();
    for i in 0..arr.len() + 1 {
        s.insert(T::from(i).unwrap_or_else(|| panic!("cannot convert {i}")));
    }
    for x in arr {
        s.remove(x);
    }
    s.pop_first()
        .unwrap_or_else(|| panic!("Some unknown error in mex_using_set"))
}

/// Finds the MEX of the values provided in `arr` using sorting.
///
/// `O(n-log(n))` implementation
pub fn mex_using_sort<T: PrimInt + Signed>(arr: &[T]) -> T {
    arr.iter().sorted_unstable().fold(
        T::zero(),
        |mex, x| if *x == mex { mex + T::one() } else { mex },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mex_using_set() {
        let data = vec![
            vec![-1, 0, 1, 2, 3],
            vec![-100, 0, 1, 2, 3, 5],
            vec![-1000000, 0, 1, 2, 5],
            vec![2, 0, 1, 2, 4],
            vec![1, 2, 3, 0, 4],
            vec![0, 1, 5, 2, 4, 3],
            vec![0, 1, 2, 3, 4, 5, 6],
            vec![0, 1, 2, 3, 4, 5, 6, 7],
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        ];
        let output = vec![4, 4, 3, 3, 5, 6, 7, 8, 9];

        for (nums, expected) in data.iter().zip(output.iter()) {
            assert_eq!(mex_using_set(nums), *expected);
        }
    }

    #[test]
    fn test_mex_using_sort() {
        let data = vec![
            vec![-1, 0, 1, 2, 3],
            vec![-100, 0, 1, 2, 3, 5],
            vec![-1000000, 0, 1, 2, 5],
            vec![2, 0, 1, 2, 4],
            vec![1, 2, 3, 0, 4],
            vec![0, 1, 5, 2, 4, 3],
            vec![0, 1, 2, 3, 4, 5, 6],
            vec![0, 1, 2, 3, 4, 5, 6, 7],
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        ];
        let output = vec![4, 4, 3, 3, 5, 6, 7, 8, 9];

        for (nums, expected) in data.iter().zip(output.iter()) {
            assert_eq!(mex_using_sort(nums), *expected);
        }
    }
}
