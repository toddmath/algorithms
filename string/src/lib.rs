//! Common string algorithms.

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

mod aho_corasick;
mod anagram;
mod autocomplete_using_trie;
mod boyer_moore_search;
mod burrows_wheeler_transform;
mod duval_algorithm;
mod hamming_distance;
mod knuth_morris_pratt;
// mod jaro_winkler_distance;
mod levenshtein_distance;
mod manacher;
mod palindrome;
mod rabin_karp;
mod reverse;
mod run_length_encoding;
mod suffix_array;
mod suffix_tree;
mod z_algorithm;

pub use self::{
    aho_corasick::AhoCorasick,
    anagram::check_anagram,
    autocomplete_using_trie::Autocomplete,
    boyer_moore_search::boyer_moore_search,
    burrows_wheeler_transform::{burrows_wheeler_transform, inv_burrows_wheeler_transform},
    duval_algorithm::duval_algorithm,
    hamming_distance::hamming_distance,
    knuth_morris_pratt::knuth_morris_pratt,
    // jaro_winkler_distance::jaro_winkler_distance,
    levenshtein_distance::levenshtein_distance,
    manacher::manacher,
    palindrome::is_palindrome,
    rabin_karp::rabin_karp,
    reverse::reverse,
    run_length_encoding::{run_length_decoding, run_length_encoding},
    suffix_array::generate_suffix_array,
    suffix_tree::{Node, SuffixTree},
    z_algorithm::{match_pattern, z_array},
};
