/// Odd even sort
pub fn odd_even_sort<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    let len = arr.len();
    loop {
        let mut sorted = true;
        for start in [1, 0] {
            for i in (start..len - 1).step_by(2) {
                if arr[i] > arr[i + 1] {
                    arr.swap(i, i + 1);
                    sorted = false;
                }
            }
            if sorted {
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn basic() {
        let mut arr = vec![3, 5, 1, 2, 4, 6];
        let cloned = arr.clone();
        odd_even_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn empty() {
        let mut arr = Vec::<i32>::new();
        let cloned = arr.clone();
        odd_even_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn one_element() {
        let mut arr = vec![3];
        let cloned = arr.clone();
        odd_even_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn pre_sorted() {
        let mut arr = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let cloned = arr.clone();
        odd_even_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }
}
