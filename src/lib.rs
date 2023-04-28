//! Common algorithms implemented in rust

#![allow(dead_code, unused_imports)]
#![warn(
    missing_docs,
    // missing_doc_code_examples
    rustdoc::broken_intra_doc_links,
)]
#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    // unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

pub mod backtracking;
pub mod big_integer;
#[allow(unused_imports)]
pub mod ciphers;
pub mod compression;
pub mod data_structures;
pub mod math;
