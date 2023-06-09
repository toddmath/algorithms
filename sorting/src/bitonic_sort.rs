// use itertools::Itertools;

/// Compare and swap
pub fn comp_and_swap<T: Ord>(array: &mut [T], low: i32, high: i32, direction: i32) {
    let (low, high) = (low as usize, high as usize);
    if (direction == 1 && array[low] > array[high]) || (direction == 0 && array[low] < array[high])
    {
        array.swap(low, high);
    }
}

/// Bitonic merge
pub fn bitonic_merge<T: Ord>(array: &mut [T], low: i32, length: i32, direction: i32) {
    if length > 1 {
        let middle = length / 2;
        for i in low..low + middle {
            comp_and_swap(array, i, i + middle, direction);
        }
        bitonic_merge(array, low, middle, direction);
        bitonic_merge(array, low + middle, middle, direction);
    }
}

/// Bitnoic sort
pub fn bitonic_sort<T: Ord>(array: &mut [T], low: i32, length: i32, direction: i32) {
    if length > 1 {
        let middle = length / 2;
        bitonic_sort(array, low, middle, 1);
        bitonic_sort(array, low + middle, middle, 0);
        bitonic_merge(array, low, length, direction);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn descending() {
        // descending
        let mut ve1 = vec![6, 5, 4, 3];
        let cloned = ve1.clone();
        bitonic_sort(&mut ve1, 0, 4, 1);
        // dbg!(&ve1, &cloned);
        assert!(is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }

    // #[test]
    // fn ascending() {
    //     // pre-sorted
    //     let mut ve2 = vec![1, 2, 3, 4];
    //     let cloned = ve2.clone();
    //     bitonic_sort(&mut ve2, 0, 4, 0);
    //     // dbg!(&ve2, &cloned);
    //     // assert!(is_sorted(&ve2));
    //     // assert!(have_same_elements(&ve2, &cloned));
    //     assert!(is_sorted(&ve2) && have_same_elements(&ve2, &cloned));
    // }
}
