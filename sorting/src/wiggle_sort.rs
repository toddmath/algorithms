//! Wiggle Sort.
//! Given an unsorted array nums, reorder it such
//! that nums[0] < nums[1] > nums[2] < nums[3]....
//! For example:
//! if input numbers = [3, 5, 2, 1, 6, 4]
//! one possible Wiggle Sorted answer is [3, 5, 1, 6, 2, 4].

use num_traits::PrimInt;

/// Wiggle Sort.
/// Given an unsorted array nums, reorder it such
/// that nums[0] < nums[1] > nums[2] < nums[3]....
///
/// # Examples
/// if input numbers = [3, 5, 2, 1, 6, 4]
/// one possible Wiggle Sorted answer is [3, 5, 1, 6, 2, 4].
pub fn wiggle_sort<T: PrimInt + Clone>(nums: &[T]) -> Vec<T> {
    let len = nums.len();
    let mut nums = nums.to_vec();

    for i in 1..len {
        let num_x = nums[i - 1];
        let num_y = nums[i];
        if (i % 2 == 1) == (num_x > num_y) {
            nums[i - 1] = num_y;
            nums[i] = num_x;
        }
    }
    nums
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn wingle_elements() {
        let arr = vec![3, 5, 2, 1, 6, 4];
        let cloned = arr;
        let res = wiggle_sort(&cloned);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn odd_number_of_elements() {
        let arr = vec![4, 1, 3, 5, 2];
        let cloned = arr;
        let res = wiggle_sort(&cloned);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn repeated_elements() {
        let arr = vec![5, 5, 5, 5];
        let cloned = arr;
        let res = wiggle_sort(&cloned);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }
}
