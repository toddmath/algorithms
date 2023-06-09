//! In place counting sort for collections of u32
//! O(n + maxval) in time, where maxval is the biggest value an input can
//! possibly take O(maxval) in memory
//! u32 is chosen arbitrarly, a counting sort probably should'nt be used on data
//! that requires bigger types.

use std::ops::AddAssign;

/// Counting sort
pub fn counting_sort(arr: &mut [u32], max: usize) {
    let mut occurences = vec![0usize; max + 1];

    for &data in arr.iter() {
        occurences[data as usize] += 1;
    }

    let mut i = 0;
    for (data, &number) in occurences.iter().enumerate() {
        for _ in 0..number {
            arr[i] = data as u32;
            i += 1;
        }
    }
}

/// Generic implementation of a counting sort for all usigned types
pub fn generic_counting_sort<T: Into<u64> + From<u8> + AddAssign + Copy>(
    arr: &mut [T],
    max: usize,
) {
    let mut occurences = vec![0usize; max + 1];
    for &data in arr.iter() {
        occurences[data.into() as usize] += 1;
    }

    let mut i = 0;
    let mut data = T::from(0);
    for &number in occurences.iter() {
        for _ in 0..number {
            arr[i] = data;
            i += 1;
        }
        data += T::from(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn counting_sort_descending() {
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        let cloned = ve1.clone();
        counting_sort(&mut ve1, 6);

        assert!(is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }

    #[test]
    fn counting_sort_pre_sorted() {
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        let cloned = ve2.clone();
        counting_sort(&mut ve2, 6);

        assert!(is_sorted(&ve2) && have_same_elements(&ve2, &cloned));
    }

    #[test]
    fn generic_counting_sort() {
        let mut ve1: Vec<u8> = vec![100, 30, 60, 10, 20, 120, 1];
        let cloned = ve1.clone();
        super::generic_counting_sort(&mut ve1, 120);

        assert!(is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }

    #[test]
    fn presorted_u64_counting_sort() {
        let mut ve2: Vec<u64> = vec![1, 2, 3, 4, 5, 6];
        let cloned = ve2.clone();
        super::generic_counting_sort(&mut ve2, 6);

        assert!(is_sorted(&ve2) && have_same_elements(&ve2, &cloned));
    }
}
