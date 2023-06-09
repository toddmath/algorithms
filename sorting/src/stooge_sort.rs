/// Stooge sort
pub fn stooge_sort<T: Ord>(arr: &mut [T]) {
    #[inline]
    fn sort<T: Ord>(arr: &mut [T], start: usize, end: usize) {
        if arr[start] > arr[end] {
            arr.swap(start, end);
        }
        if end > start + 1 {
            let k = (end - start + 1) / 3;
            sort(arr, start, end - k);
            sort(arr, start + k, end);
            sort(arr, start, end - k);
        }
    }

    if arr.is_empty() {
        return;
    }
    sort(arr, 0, arr.len() - 1);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn basic() {
        let mut vec = vec![3, 5, 6, 3, 1, 4];
        let cloned = vec.clone();
        stooge_sort(&mut vec);
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }

    #[test]
    fn empty() {
        let mut vec: Vec<i32> = vec![];
        let cloned = vec.clone();
        stooge_sort(&mut vec);
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }

    #[test]
    fn reverse() {
        let mut vec = vec![6, 5, 4, 3, 2, 1];
        let cloned = vec.clone();
        stooge_sort(&mut vec);
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }

    #[test]
    fn already_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        let cloned = vec.clone();
        stooge_sort(&mut vec);
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }
}
