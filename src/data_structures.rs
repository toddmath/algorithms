//! Common data structures.

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
