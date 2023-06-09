use std::cmp::Ordering;

use sorting::partition;

/// Returns k-th smallest element of an array, i.e. its order statistics.
/// Time complexity is O(n^2) in the worst case, but only O(n) on average.
/// It mutates the input, and therefore does not require additional space.
pub fn kth_smallest<T: Ord + Copy>(input: &mut [T], k: usize) -> Option<T> {
    fn smallest<T: Ord + Copy>(input: &mut [T], k: usize, lo: usize, hi: usize) -> T {
        if lo == hi {
            return input[lo];
        }
        let pivot = partition(input, lo as isize, hi as isize) as usize;
        let i = pivot - lo + 1;
        match k.cmp(&i) {
            Ordering::Equal => input[pivot],
            Ordering::Less => smallest(input, k, lo, pivot - 1),
            Ordering::Greater => smallest(input, k - i, pivot + 1, hi),
        }
    }

    if input.is_empty() {
        None
    } else {
        let kth = smallest(input, k, 0, input.len() - 1);
        Some(kth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut zero: [u8; 0] = [];
        let first = kth_smallest(&mut zero, 1);

        assert_eq!(None, first);
    }

    #[test]
    fn one_element() {
        let mut one = [1];
        let first = kth_smallest(&mut one, 1);

        assert_eq!(1, first.unwrap());
    }

    #[test]
    fn many_elements() {
        // 0 1 3 4 5 7 8 9 9 10 12 13 16 17
        let mut many = [9, 17, 3, 16, 13, 10, 1, 5, 7, 12, 4, 8, 9, 0];

        let first = kth_smallest(&mut many, 1);
        let third = kth_smallest(&mut many, 3);
        let sixth = kth_smallest(&mut many, 6);
        let fourteenth = kth_smallest(&mut many, 14);

        assert_eq!(0, first.unwrap());
        assert_eq!(3, third.unwrap());
        assert_eq!(7, sixth.unwrap());
        assert_eq!(17, fourteenth.unwrap());
    }
}
