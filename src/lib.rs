//! Common algorithms implemented in rust

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

#[cfg(not(feature = "once_cell"))]
pub use std::{cell::OnceCell, sync::Once as OnceCellSync};

#[cfg(feature = "once_cell")]
pub use once_cell::{sync::Lazy as OnceCellSync, unsync::Lazy as OnceCell};

pub mod backtracking;
pub mod big_integer;
pub mod ciphers;
pub mod compression;
pub mod data_structures;
pub mod math;
