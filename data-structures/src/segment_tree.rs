use std::{cmp, fmt::Debug, ops::Range};

// use itertools::Itertools;
use num_traits::PrimInt;

/// This data structure implements a segment-tree that can efficiently answer
/// range (interval) queries on arrays. It represents this array as a binary
/// tree of merged intervals. From top to bottom: [aggregated value for the
/// overall array], then [left-hand half, right hand half], etc. until [each
/// individual value, ...] It is generic over a reduction function for each
/// segment or interval: basically, to describe how we merge two intervals
/// together.
///
/// # Note
/// This function should be commutative and associative.
/// It could be:
/// - `std::cmp::min(interval_1, interval_2)`
/// - `std::cmp::max(interval_1, interval_2)`
/// - `|a, b| a + b`, `|a, b| a * b`
#[derive(Debug)]
pub struct SegmentTree<T: PrimInt + Debug + Default, F: Fn(T, T) -> T> {
    len: usize,
    tree: Vec<T>,
    merge: F,
}

impl<T: PrimInt + Debug + Default, F: Fn(T, T) -> T> SegmentTree<T, F> {
    /// Builds a SegmentTree from an array and a merge function
    pub fn new(arr: &[T], merge: F) -> Self {
        let len = arr.len();
        let mut tree = vec![T::default(); 2 * len];
        tree[len..(2 * len)].clone_from_slice(&arr[0..len]);
        for i in (1..len).rev() {
            tree[i] = merge(tree[2 * i], tree[2 * i + 1]);
        }
        SegmentTree { len, tree, merge }
    }

    /// Query the range (exclusive)
    /// returns None if the range is out of the array's boundaries (eg: if start
    /// is after the end of the array, or start > end, etc.) return the
    /// aggregate of values over this range otherwise
    pub fn query(&self, range: Range<usize>) -> Option<T> {
        let mut l = range.start + self.len;
        let mut r = cmp::min(self.len, range.end) + self.len;
        let mut res = None;

        while l < r {
            if l % 2 == 1 {
                res = res.map_or(Some(self.tree[l]), |old| {
                    Some((self.merge)(old, self.tree[l]))
                });
                // res = Some(match res {
                //     None => self.tree[l],
                //     Some(old) => (self.merge)(old, self.tree[l]),
                // });
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                res = res.map_or(Some(self.tree[r]), |old| {
                    Some((self.merge)(old, self.tree[r]))
                });
            }
            l /= 2;
            r /= 2;
        }
        res
    }

    /// Updates the value at index `idx` in the original array with a new value
    /// `val`
    pub fn update(&mut self, idx: usize, val: T) {
        let mut idx = idx + self.len;
        self.tree[idx] = val;
        idx /= 2;
        while idx != 0 {
            self.tree[idx] = (self.merge)(self.tree[2 * idx], self.tree[2 * idx + 1]);
            idx /= 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::{max, min};

    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    use super::*;

    #[test]
    fn test_min_segments() {
        let vec = vec![-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        let min_seg_tree = SegmentTree::new(&vec, min);
        assert_eq!(Some(-5), min_seg_tree.query(4..7));
        assert_eq!(Some(-30), min_seg_tree.query(0..vec.len()));
        assert_eq!(Some(-30), min_seg_tree.query(0..2));
        assert_eq!(Some(-4), min_seg_tree.query(1..3));
        assert_eq!(Some(-5), min_seg_tree.query(1..7));
    }

    #[test]
    fn test_max_segments() {
        let val_at_6 = 6;
        let vec = vec![1, 2, -4, 7, 3, -5, val_at_6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut max_seg_tree = SegmentTree::new(&vec, max);
        assert_eq!(Some(15), max_seg_tree.query(0..vec.len()));
        let max_4_to_6 = 6;
        assert_eq!(Some(max_4_to_6), max_seg_tree.query(4..7));
        let delta = 2;
        max_seg_tree.update(6, val_at_6 + delta);
        assert_eq!(Some(val_at_6 + delta), max_seg_tree.query(4..7));
    }

    #[test]
    fn test_sum_segments() {
        let val_at_6 = 6;
        let vec = vec![1, 2, -4, 7, 3, -5, val_at_6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut sum_seg_tree = SegmentTree::new(&vec, |a, b| a + b);
        for (i, val) in vec.iter().enumerate() {
            assert_eq!(Some(*val), sum_seg_tree.query(i..(i + 1)));
        }
        let sum_4_to_6 = sum_seg_tree.query(4..7);
        assert_eq!(Some(4), sum_4_to_6);
        let delta = 3;
        sum_seg_tree.update(6, val_at_6 + delta);
        assert_eq!(
            sum_4_to_6.unwrap() + delta,
            sum_seg_tree.query(4..7).unwrap()
        );
    }

    // Some properties over segment trees:
    //  When asking for the range of the overall array, return the same as
    // iter().min() or iter().max(), etc.  When asking for an interval
    // containing a single value, return this value, no matter the merge function

    #[quickcheck]
    fn check_overall_interval_min(array: Vec<i32>) -> TestResult {
        let seg_tree = SegmentTree::new(&array, min);
        TestResult::from_bool(array.iter().min().copied() == seg_tree.query(0..array.len()))
    }

    #[quickcheck]
    fn check_overall_interval_max(array: Vec<i32>) -> TestResult {
        let seg_tree = SegmentTree::new(&array, max);
        TestResult::from_bool(array.iter().max().copied() == seg_tree.query(0..array.len()))
    }

    #[quickcheck]
    fn check_overall_interval_sum(array: Vec<i32>) -> TestResult {
        let seg_tree = SegmentTree::new(&array, max);
        TestResult::from_bool(array.iter().max().copied() == seg_tree.query(0..array.len()))
    }

    #[quickcheck]
    fn check_single_interval_min(array: Vec<i32>) -> TestResult {
        let seg_tree = SegmentTree::new(&array, min);
        for (i, value) in array.into_iter().enumerate() {
            let res = seg_tree.query(i..(i + 1));
            if res != Some(value) {
                return TestResult::error(format!("Expected {:?}, got {:?}", Some(value), res));
            }
        }
        TestResult::passed()
    }

    #[quickcheck]
    fn check_single_interval_max(array: Vec<i32>) -> TestResult {
        let seg_tree = SegmentTree::new(&array, max);
        for (i, value) in array.into_iter().enumerate() {
            let res = seg_tree.query(i..(i + 1));
            if res != Some(value) {
                return TestResult::error(format!("Expected {:?}, got {:?}", Some(value), res));
            }
        }
        TestResult::passed()
    }

    #[quickcheck]
    fn check_single_interval_sum(array: Vec<i32>) -> TestResult {
        let seg_tree = SegmentTree::new(&array, max);
        for (i, value) in array.into_iter().enumerate() {
            let res = seg_tree.query(i..(i + 1));
            if res != Some(value) {
                return TestResult::error(format!("Expected {:?}, got {:?}", Some(value), res));
            }
        }
        TestResult::passed()
    }
}
