/// Partition for [`quick_sort`]
pub fn partition<T>(arr: &mut [T], lo: isize, hi: isize) -> isize
where
    T: Ord,
{
    let pivot = hi as usize;
    let mut i = lo - 1;
    let mut j = hi;

    while i < j {
        i += 1;
        while arr[i as usize] < arr[pivot] {
            i += 1;
        }

        j -= 1;
        while j >= 0 && arr[j as usize] > arr[pivot] {
            j -= 1;
        }

        if i >= j {
            break;
        } else {
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap(i as usize, pivot);
    i
}

/// Quick sort
pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    #[inline]
    fn sort<S: Ord>(arr: &mut [S], lo: isize, hi: isize) {
        if lo < hi {
            let p = partition(arr, lo, hi);
            sort(arr, lo, p - 1);
            sort(arr, p + 1, hi);
        }
    }

    let len = arr.len();
    if len > 1 {
        sort(arr, 0, (len - 1) as isize);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn basic() {
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        let cloned = res.clone();
        quick_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        let cloned = res.clone();
        quick_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        let cloned = res.clone();
        quick_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        let cloned = res.clone();
        quick_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        let cloned = res.clone();
        quick_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        let cloned = res.clone();
        quick_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }
}
