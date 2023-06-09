fn merge<T: Ord + Copy>(arr: &mut [T], mid: usize) {
    debug_assert!(mid < arr.len(), "mid must be less than arr.len()");
    let (left, right) = arr.split_at(mid);
    let left = left.to_vec();
    let right = right.to_vec();
    let mut l = 0;
    let mut r = 0;

    for x in arr {
        if r == right.len() || (l < left.len() && left[l] < right[r]) {
            *x = left[l];
            l += 1;
        } else {
            *x = right[r];
            r += 1;
        }
    }
}

/// Top-down merge sort
pub fn top_down_merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let mid = arr.len() / 2;
    top_down_merge_sort(&mut arr[..mid]);
    top_down_merge_sort(&mut arr[mid..]);
    merge(arr, mid);
}

/// Bottom-up merge sort
pub fn bottom_up_merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let len = arr.len();
    let mut sub_array_size = 1usize;
    while sub_array_size < len {
        let mut start_index = 0usize;
        while len - start_index > sub_array_size {
            let end_index = if start_index + 2 * sub_array_size > len {
                len
            } else {
                start_index + 2 * sub_array_size
            };
            merge(&mut arr[start_index..end_index], sub_array_size);
            start_index = end_index;
        }
        sub_array_size *= 2;
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod top_down_merge_sort {
        use super::super::*;
        use crate::{have_same_elements, is_sorted};

        #[test]
        fn basic() {
            let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn basic_string() {
            let mut res = vec!["a", "bb", "d", "cc"];
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn empty() {
            let mut res = Vec::<u8>::new();
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn one_element() {
            let mut res = vec![1];
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn pre_sorted() {
            let mut res = vec![1, 2, 3, 4];
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn reverse_sorted() {
            let mut res = vec![4, 3, 2, 1];
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }
    }

    #[cfg(test)]
    mod bottom_up_merge_sort {
        use super::super::*;
        use crate::{have_same_elements, is_sorted};

        #[test]
        fn basic() {
            let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn basic_string() {
            let mut res = vec!["a", "bb", "d", "cc"];
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn empty() {
            let mut res = Vec::<u8>::new();
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn one_element() {
            let mut res = vec![1];
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn pre_sorted() {
            let mut res = vec![1, 2, 3, 4];
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn reverse_sorted() {
            let mut res = vec![4, 3, 2, 1];
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }
    }
}
