use std::{collections::HashMap, hash::Hash};

use num_traits::PrimInt;

/// Given an array of integers nums and an integer target,
/// return indices of the two numbers such that they add up to target.
///
/// # Parameters
///
/// - `nums`: A list of numbers to check.
/// - `target`: The target sum.
///
/// # Returns
///
/// If the target sum is found in the array, the indices of the augend and
/// addend are returned as a tuple.
///
/// If the target sum cannot be found in the array, `None` is returned.
pub fn two_sum<T: PrimInt + Hash>(nums: Vec<T>, target: T) -> Option<(usize, usize)> {
    let mut distance_table = HashMap::new();

    for (i, value) in nums.iter().enumerate() {
        match distance_table.get(&(target - *value)) {
            Some(j) => return Some((i, *j)),
            _ => distance_table.insert(*value, i),
        };
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let nums = vec![2, 7, 11, 15];
        assert_eq!(two_sum(nums, 9), Some((1, 0)));

        let nums = vec![3, 2, 4];
        assert_eq!(two_sum(nums, 6), Some((2, 1)));

        let nums = vec![3, 3];
        assert_eq!(two_sum(nums, 6), Some((1, 0)));

        let nums = vec![2, 7, 11, 15];
        assert_eq!(two_sum(nums, 16), None);
    }
}
