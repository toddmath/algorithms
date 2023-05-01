//! # Big Integer
//!
//! Some common examples of big integer algorithms.

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

mod hello_bigmath;
mod poly1305;

pub use self::{hello_bigmath::factorial, poly1305::Poly1305};
