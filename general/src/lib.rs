//! General algorithms.

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

mod convex_hull;
mod fisher_yates_shuffle;
mod genetic;
mod hanoi;
mod huffman_encoding;
mod kmeans;
mod mex;
mod permutations;
mod two_sum;

pub use self::{
    convex_hull::convex_hull_graham,
    fisher_yates_shuffle::fisher_yates_shuffle,
    genetic::{
        Chromosome, GenericAlgorithmParams, GeneticAlgorithm, RouletteWheel, SelectionStrategy,
        Tournament,
    },
    hanoi::hanoi,
    huffman_encoding::{HuffmanDictionary, HuffmanEncoding, HuffmanNode, HuffmanValue},
    kmeans::{f32::kmeans as kmeans_f32, f64::kmeans as kmeans_f64},
    mex::{mex_using_set, mex_using_sort},
    permutations::{heap_permute, permute, permute_unique, steinhaus_johnson_trotter_permute},
    two_sum::two_sum,
};
