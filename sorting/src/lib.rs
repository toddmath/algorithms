//! Sorting algorithms

#![feature(is_sorted)]
#![warn(
    missing_docs,
    // missing_doc_code_examples
    rustdoc::broken_intra_doc_links,
    // unstable_features,
)]
#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]

mod bead_sort;
mod bitonic_sort;
mod bogo_sort;
mod bubble_sort;
mod bucket_sort;
mod cocktail_shaker_sort;
mod comb_sort;
mod counting_sort;
mod cycle_sort;
mod dutch_national_flag_sort;
mod exchange_sort;
mod gnome_sort;
mod heap_sort;
mod insertion_sort;
mod merge_sort;
mod odd_even_sort;
mod pancake_sort;
mod patience_sort;
mod pigeonhole_sort;
mod quick_sort;
mod radix_sort;
mod selection_sort;
mod shell_sort;
mod sleep_sort;
mod stooge_sort;
mod tim_sort;
mod wiggle_sort;

#[cfg(test)]
use std::hash::Hash;

pub use self::{
    bead_sort::bead_sort,
    bitonic_sort::{bitonic_merge, bitonic_sort, comp_and_swap},
    bogo_sort::bogo_sort,
    bubble_sort::bubble_sort,
    bucket_sort::bucket_sort,
    cocktail_shaker_sort::cocktail_shaker_sort,
    comb_sort::comb_sort,
    counting_sort::{counting_sort, generic_counting_sort},
    cycle_sort::cycle_sort,
    dutch_national_flag_sort::{dutch_national_flag_sort, Colors},
    exchange_sort::exchange_sort,
    gnome_sort::gnome_sort,
    heap_sort::heap_sort,
    insertion_sort::insertion_sort,
    merge_sort::{bottom_up_merge_sort, top_down_merge_sort},
    odd_even_sort::odd_even_sort,
    pancake_sort::pancake_sort,
    patience_sort::patience_sort,
    pigeonhole_sort::pigeonhole_sort,
    quick_sort::{partition, quick_sort},
    radix_sort::radix_sort,
    selection_sort::selection_sort,
    shell_sort::shell_sort,
    sleep_sort::sleep_sort,
    stooge_sort::stooge_sort,
    tim_sort::tim_sort,
    wiggle_sort::wiggle_sort,
};

#[cfg(test)]
#[inline]
pub fn have_same_elements<T>(a: &[T], b: &[T]) -> bool
where
    T: Eq + Hash,
{
    use std::collections::HashSet;
    if a.len() != b.len() {
        return false;
    }
    let set_a: HashSet<_> = a.iter().collect();
    let set_b: HashSet<_> = b.iter().collect();
    set_a == set_b
}

#[cfg(test)]
#[inline]
pub fn is_sorted<T>(arr: &[T]) -> bool
where
    T: PartialOrd,
{
    // use itertools::Itertools;

    arr.is_sorted()
    // arr.iter().tuple_windows().all(|(a, b)| a <= b)
    // arr.windows(2).all(|w| w[0] <= w[1])
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
