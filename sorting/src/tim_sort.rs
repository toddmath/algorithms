use std::cmp;

static MIN_MERGE: usize = 32;

#[inline]
fn min_run_length(mut n: usize) -> usize {
    let mut r = 0;
    while n >= MIN_MERGE {
        r |= n & 1;
        n >>= 1;
    }
    n + r
}

fn insertion_sort(arr: &mut Vec<i32>, left: usize, right: usize) -> &Vec<i32> {
    for i in (left + 1)..(right + 1) {
        let temp = arr[i];
        let mut j = (i - 1) as i32;

        while j >= left as i32 && arr[j as usize] > temp {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }
        arr[(j + 1) as usize] = temp;
    }
    arr
}

fn merge(arr: &mut Vec<i32>, l: usize, m: usize, r: usize) -> &Vec<i32> {
    let (llen, rlen) = (m - l + 1, r - m);
    let (mut left, mut right) = (vec![0; llen], vec![0; rlen]);
    left[..llen].clone_from_slice(&arr[l..(llen + 1)]);
    right[..rlen].clone_from_slice(&arr[(m + 1)..(rlen + 1)]);

    let (mut i, mut j, mut k) = (0, 0, l);

    while i < llen && j < rlen {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < llen {
        arr[k] = left[i];
        k += 1;
        i += 1;
    }
    while j < rlen {
        arr[k] = right[j];
        k += 1;
        j += 1;
    }
    arr
}

/// Tim sort
pub fn tim_sort(arr: &mut Vec<i32>, n: usize) {
    let min_run = min_run_length(MIN_MERGE);

    for (left, right) in (0..n).step_by(min_run).zip(
        (0..n)
            .step_by(min_run)
            .map(|x| cmp::min(x + MIN_MERGE - 1, n - 1)),
    ) {
        insertion_sort(arr, left, right);
    }

    for (left, mid, right) in (min_run..n).flat_map(|mut size| {
        size *= 2;
        (0..n)
            .map(move |x| x + 2 * size)
            .map(move |left| (left, left + size - 1, cmp::min(left + 2 * size - 1, n - 1)))
            .filter(|&(_, mid, right)| mid < right)
    }) {
        merge(arr, left, mid, right);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn basic() {
        let mut array = vec![-2, 7, 15, -14, 0, 15, 0, 7, -7, -4, -13, 5, 8, -14, 12];
        let cloned = array.clone();
        let arr_len = array.len();
        tim_sort(&mut array, arr_len);
        assert!(is_sorted(&array) && have_same_elements(&array, &cloned));
    }

    #[test]
    fn empty() {
        let mut array = Vec::<i32>::new();
        let cloned = array.clone();
        let arr_len = array.len();
        tim_sort(&mut array, arr_len);
        assert!(is_sorted(&array) && have_same_elements(&array, &cloned));
    }

    #[test]
    fn one_element() {
        let mut array = vec![3];
        let cloned = array.clone();
        let arr_len = array.len();
        tim_sort(&mut array, arr_len);
        assert!(is_sorted(&array) && have_same_elements(&array, &cloned));
    }

    #[test]
    fn pre_sorted() {
        let mut array = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let cloned = array.clone();
        let arr_len = array.len();
        tim_sort(&mut array, arr_len);
        assert!(is_sorted(&array) && have_same_elements(&array, &cloned));
    }
}
