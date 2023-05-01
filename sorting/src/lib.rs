//! Sorting algorithms

#![feature(is_sorted)]

mod bead_sort;
mod bogo_sort;
mod bubble_sort;
mod bucket_sort;
mod cocktail_shaker_sort;
mod comb_sort;
// mod counting_sort;
// mod cycle_sort;
// mod dutch_national_flag_sort;
// mod exchange_sort;
// mod gnome_sort;
// mod heap_sort;
mod insertion_sort;
// mod merge_sort;
// mod odd_even_sort;
// mod pancake_sort;
// mod patience_sort;
// mod pigeonhole_sort;
// mod quick_sort;
// mod radix_sort;
// mod selection_sort;
// mod shell_sort;
// mod sleep_sort;
// mod stooge_sort;
// mod tim_sort;

#[cfg(test)]
use std::hash::Hash;

pub use self::{
    bead_sort::bead_sort, bogo_sort::bogo_sort, bubble_sort::bubble_sort, bucket_sort::bucket_sort,
    cocktail_shaker_sort::cocktail_shaker_sort, comb_sort::comb_sort,
    insertion_sort::insertion_sort,
};

#[cfg(test)]
#[inline]
pub fn have_same_elements<T>(a: &[T], b: &[T]) -> bool
where
    T: Eq + Hash,
{
    use std::collections::HashSet;
    // use itertools::Itertools;
    if a.len() != b.len() {
        return false;
    }
    let set_a: HashSet<&T> = a.iter().collect();
    let set_b: HashSet<&T> = b.iter().collect();
    set_a == set_b
}

#[cfg(test)]
#[inline]
pub fn is_sorted<T>(arr: &[T]) -> bool
where
    T: PartialOrd,
{
    arr.windows(2).all(|w| w[0] <= w[1])
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_sorted() {
        use super::*;

        assert!(is_sorted::<isize>(&[]));
        assert!(is_sorted(&["a"]));
        assert!(is_sorted(&[1, 2, 3]));
        assert!(is_sorted(&[0, 1, 1]));

        assert!(!is_sorted(&[1, 0]));
        assert!(!is_sorted(&[2, 3, 1, -1, 5]));
    }
}
