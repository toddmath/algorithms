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
// mod rb_tree;
mod binary_search_tree;
mod fenwick_tree;
mod graph;
mod heap;
mod linked_list;
mod probabilistic;
mod queue;
mod segment_tree;
mod stack_using_singly_linked_list;
mod treap;
mod trie;
mod union_find;

pub use self::{
    avl_tree::AVLTree,
    b_tree::BTree,
    binary_search_tree::BinarySearchTree,
    fenwick_tree::FenwickTree,
    graph::{DirectedGraph, Graph, NodeNotInGraph, UndirectedGraph},
    heap::Heap,
    linked_list::LinkedList,
    probabilistic::{
        bloom_filter::{
            BasicBloomFilter, BloomFilter, MultiBinaryBloomFilter, SingleBinaryBloomFilter,
        },
        count_min_sketch::{CountMinSketch, HashCountMinSketch},
    },
    queue::Queue,
    segment_tree::SegmentTree,
    stack_using_singly_linked_list::Stack,
    treap::Treap,
    trie::Trie,
    union_find::UnionFind,
};
