//! [`BTree`] data structure.

use std::{convert::TryFrom, fmt::Debug, mem};

#[derive(Debug, Default)]
struct Node<T> {
    keys: Vec<T>,
    children: Vec<Node<T>>,
}

#[derive(Debug, Default)]
/// [`BTree`] data structure.
pub struct BTree<T> {
    root: Node<T>,
    props: BTreeProps,
}

#[derive(Debug, Default)]
/// [Why this is necessary](http://smallcultfollowing.com/babysteps/blog/2018/11/01/after-nll-interprocedural-conflicts/#fnref:improvement)
struct BTreeProps {
    degree: usize,
    max_keys: usize,
    mid_key_index: usize,
}

impl<T> Node<T>
where
    T: Ord,
{
    fn new(degree: usize, keys: Option<Vec<T>>, children: Option<Vec<Node<T>>>) -> Self {
        Node {
            keys: keys.unwrap_or_else(|| Vec::with_capacity(degree - 1)),
            children: children.unwrap_or_else(|| Vec::with_capacity(degree)),
        }
    }

    #[inline(always)]
    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

impl BTreeProps {
    fn new(degree: usize) -> Self {
        Self {
            degree,
            max_keys: degree - 1,
            mid_key_index: (degree - 1) / 2,
        }
    }

    #[inline(always)]
    fn is_maxed_out<T: Ord + Copy>(&self, node: &Node<T>) -> bool {
        node.keys.len() == self.max_keys
    }

    fn split_child<T: Ord + Copy + Default>(&self, parent: &mut Node<T>, child_index: usize) {
        let child = &mut parent.children[child_index];
        let mid_key = child.keys[self.mid_key_index];

        let right_keys = child
            .keys
            .split_off(self.mid_key_index)
            .split_first()
            .map_or_else(
                || Vec::with_capacity(self.max_keys),
                |(_, others)| others.to_vec(),
            );

        let right_children =
            (!child.is_leaf()).then(|| child.children.split_off(self.mid_key_index + 1));

        let new_child_node = Node::new(self.degree, Some(right_keys), right_children);

        parent.keys.insert(child_index, mid_key);
        parent.children.insert(child_index + 1, new_child_node);
    }

    fn insert_non_full<T: Ord + Copy + Default>(&mut self, node: &mut Node<T>, key: T) {
        let mut index = isize::try_from(node.keys.len()).ok().unwrap() - 1;
        while index >= 0 && node.keys[index as usize] >= key {
            index -= 1;
        }
        let mut index = usize::try_from(index + 1).ok().unwrap();

        if node.is_leaf() {
            node.keys.insert(index, key);
        } else {
            if self.is_maxed_out(&node.children[index]) {
                self.split_child(node, index);
                if node.keys[index] < key {
                    index += 1;
                }
            }
            self.insert_non_full(&mut node.children[index], key);
        }
    }

    fn traverse_node<T: Ord + Debug>(node: &Node<T>, depth: usize) {
        if node.is_leaf() {
            print!(" {0:{<1$}{2:?}{0:}<1$} ", "", depth, node.keys);
        } else {
            let depth = depth + 1;
            for (index, key) in node.keys.iter().enumerate() {
                Self::traverse_node(&node.children[index], depth);
                // Check https://doc.rust-lang.org/std/fmt/index.html
                // And https://stackoverflow.com/a/35280799/2849127
                print!("{0:{<1$}{2:?}{0:}<1$}", "", depth, key);
            }
            Self::traverse_node(node.children.last().unwrap(), depth);
        }
    }
}

impl<T> BTree<T>
where
    T: Ord + Copy + Debug + Default,
{
    /// Creates a new [`BTree<T>`].
    pub fn new(branch_factor: usize) -> Self {
        let degree = 2 * branch_factor;
        Self {
            root: Node::new(degree, None, None),
            props: BTreeProps::new(degree),
        }
    }

    /// Inserts a key into the [`BTree<T>`].
    pub fn insert(&mut self, key: T) {
        if self.props.is_maxed_out(&self.root) {
            let mut new_root = Node::new(self.props.degree, None, None);
            mem::swap(&mut new_root, &mut self.root);
            self.root.children.insert(0, new_root);
            self.props.split_child(&mut self.root, 0);
        }
        self.props.insert_non_full(&mut self.root, key);
    }

    /// Traverses the [`BTree<T>`].
    pub fn traverse(&self) {
        BTreeProps::traverse_node(&self.root, 0);
        println!();
    }

    /// Search the [`BTree<T>`] for a key.
    pub fn search(&self, key: T) -> bool {
        let mut current_node = &self.root;
        let mut index: isize;
        loop {
            index = isize::try_from(current_node.keys.len()).ok().unwrap() - 1;
            while index >= 0 && current_node.keys[index as usize] > key {
                index -= 1;
            }
            let u_index = usize::try_from(index + 1).ok().unwrap();
            #[allow(unused_comparisons, clippy::absurd_extreme_comparisons)]
            if index >= 0 && current_node.keys[u_index - 1] == key {
                break true;
            } else if current_node.is_leaf() {
                break false;
            } else {
                current_node = &current_node.children[u_index];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search() {
        let mut tree = BTree::new(2);
        tree.insert(10);
        tree.insert(20);
        tree.insert(30);
        tree.insert(5);
        tree.insert(6);
        tree.insert(7);
        tree.insert(11);
        tree.insert(12);
        tree.insert(15);
        assert!(tree.search(15));
        assert!(!tree.search(16));
    }
}
