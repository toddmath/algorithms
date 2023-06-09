/// Exchange sort
pub fn exchange_sort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    for (a, b) in (0..len).flat_map(move |i| ((i + 1)..len).map(move |j| (i, j))) {
        if arr[b] < arr[a] {
            arr.swap(a, b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn it_works() {
        let mut arr1 = [6, 5, 4, 3, 2, 1];
        let cloned = arr1;
        exchange_sort(&mut arr1);
        assert!(is_sorted(&arr1) && have_same_elements(&arr1, &cloned));

        arr1 = [12, 343, 21, 90, 3, 21];
        let cloned = arr1;
        exchange_sort(&mut arr1);
        assert!(is_sorted(&arr1) && have_same_elements(&arr1, &cloned));

        let mut arr2 = [1];
        let cloned = arr2;
        exchange_sort(&mut arr2);
        assert!(is_sorted(&arr2) && have_same_elements(&arr2, &cloned));

        let mut arr3 = [213, 542, 90, -23412, -32, 324, -34, 3324, 54];
        let cloned = arr3;
        exchange_sort(&mut arr3);
        assert!(is_sorted(&arr3) && have_same_elements(&arr3, &cloned));
    }
}
