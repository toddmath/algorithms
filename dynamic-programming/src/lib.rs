//! Dynamic Programming Algorithms.

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

mod coin_change;
mod egg_dropping;
mod fibonacci;
mod fractional_knapsack;
mod is_subsequence;
mod knapsack;
mod longest_common_subsequence;
mod longest_continuous_increasing_subsequence;
mod maximal_square;
mod maximum_subarray;
mod rod_cutting;
mod snail;
mod subset_generation;

pub use self::{
    coin_change::coin_change,
    egg_dropping::egg_drop,
    fibonacci::{
        classical_fibonacci, fibonacci, logarithmic_fibonacci, matrix_fibonacci,
        memoized_fibonacci, recursive_fibonacci,
    },
    fractional_knapsack::fractional_knapsack,
    is_subsequence::is_subsequence,
    knapsack::knapsack,
    longest_common_subsequence::longest_common_subsequence,
    longest_continuous_increasing_subsequence::longest_continuous_increasing_subsequence,
    maximal_square::maximal_square,
    maximum_subarray::maximum_subarray,
    rod_cutting::rod_cut,
    snail::snail,
    subset_generation::{list_subset, subsets},
};
