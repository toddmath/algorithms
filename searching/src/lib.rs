//! Searching algorithms.

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

mod binary_search;
mod binary_search_recursive;
mod exponential_search;
mod fibonacci_search;
mod interpolation_search;
mod jump_search;
mod kth_smallest;
mod kth_smallest_heap;
mod linear_search;
mod quick_select;
mod ternary_search;
mod ternary_search_min_max;
mod ternary_search_min_max_recursive;
mod ternary_search_recursive;

pub use self::{
    binary_search::binary_search,
    binary_search_recursive::binary_search_rec,
    exponential_search::exponential_search,
    fibonacci_search::fibonacci_search,
    interpolation_search::interpolation_search,
    jump_search::jump_search,
    kth_smallest::kth_smallest,
    kth_smallest_heap::kth_smallest_heap,
    linear_search::linear_search,
    quick_select::quick_select,
    ternary_search::ternary_search,
    ternary_search_min_max::{ternary_search_max, ternary_search_min},
    ternary_search_min_max_recursive::{ternary_search_max_rec, ternary_search_min_rec},
    ternary_search_recursive::ternary_search_rec,
};

#[cfg(test)]
#[macro_use]
extern crate is_close;
