use std::{cmp::Ordering, fmt};

// use itertools::Itertools;
use num_traits::PrimInt;

/// Interpolation search
pub fn interpolation_search<T: PrimInt + fmt::Debug>(nums: &[T], item: &T) -> Option<usize> {
    if nums.is_empty() {
        return None;
    }
    let (mut low, mut high) = (0, nums.len() - 1);
    let to_usize = |n: T| {
        n.to_usize()
            .expect(&format!("{n:?} cannot be converted to usize"))
    };

    while low <= high {
        if !(nums[low]..=nums[high]).contains(item) {
            break;
        }
        let offset =
            low + ((high - low) / to_usize(nums[high] - nums[low]) * to_usize(*item - nums[low]));

        match nums[offset].cmp(item) {
            Ordering::Equal => return Some(offset),
            Ordering::Less => low = offset + 1,
            Ordering::Greater => high = offset - 1,
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_if_empty_slice() {
        let nums = [];
        assert!(interpolation_search(&nums, &3).is_none());
    }

    #[test]
    fn none_if_target_not_found() {
        let nums = [1, 2, 3, 4, 5, 6];
        assert!(interpolation_search(&nums, &10).is_none());
    }

    #[test]
    fn first_index() {
        let index = interpolation_search(&[1, 2, 3, 4, 5], &1);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn last_index() {
        let index = interpolation_search(&[1, 2, 3, 4, 5], &5);
        assert_eq!(index, Some(4));
    }

    #[test]
    fn middle_index() {
        let index = interpolation_search(&[1, 2, 3, 4, 5], &3);
        assert_eq!(index, Some(2));
    }
}
