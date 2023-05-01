//! Common data structures.

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

mod avl_tree;
mod b_tree;
mod heap;
mod linked_list;
mod queue;
mod stack_using_singly_linked_list;
mod trie;

pub use self::{
    avl_tree::AVLTree, b_tree::BTree, heap::Heap, linked_list::LinkedList, queue::Queue,
    stack_using_singly_linked_list::Stack, trie::Trie,
};
