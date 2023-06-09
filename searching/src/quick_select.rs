//! Quick Select
//!
//! * [Wikipedia](https://en.wikipedia.org/wiki/Quickselect)

use std::cmp::Ordering;

use num_traits::PrimInt;

fn partition<T: PrimInt>(list: &mut [T], left: usize, right: usize, pivot_index: usize) -> usize {
    let pivot_value = list[pivot_index];
    list.swap(pivot_index, right);
    let mut store_index = left;
    for i in left..=right {
        if list[i] < pivot_value {
            list.swap(store_index, i);
            store_index += 1;
        }
        list.swap(right, store_index);
    }
    store_index
}

/// Quick Select
///
/// * [Wikipedia](https://en.wikipedia.org/wiki/Quickselect)
pub fn quick_select<T: PrimInt>(list: &mut [T], left: usize, right: usize, index: usize) -> T {
    if left == right {
        return list[left];
    }
    let mut pivot_index = ((left + right) / 2) + 1;
    pivot_index = partition(list, left, right, pivot_index);

    match index.cmp(&pivot_index) {
        Ordering::Equal => list[index],
        Ordering::Less => quick_select(list, left, pivot_index - 1, index),
        Ordering::Greater => quick_select(list, pivot_index + 1, right, index),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut arr1 = [2, 3, 4, 5];
        assert_eq!(quick_select(&mut arr1, 0, 3, 1), 3);
        let mut arr2 = [2, 5, 9, 12, 16];
        assert_eq!(quick_select(&mut arr2, 1, 3, 2), 12);
        let mut arr2 = [0, 3, 8];
        assert_eq!(quick_select(&mut arr2, 0, 0, 0), 0);
    }
}
