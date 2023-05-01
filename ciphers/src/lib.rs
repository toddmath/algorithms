//! Common cipher algorithms

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

mod another_rot13;
mod caesar;
mod hashing_traits;
mod rot13;
mod sha256;
mod theoretical_rot13;
mod xor;

pub use self::{
    another_rot13::another_rot13,
    caesar::caesar,
    hashing_traits::{Hasher, HMAC},
    rot13::rot13,
    sha256::SHA256,
    theoretical_rot13::theoretical_rot13,
    xor::{xor, xor_bytes},
};
