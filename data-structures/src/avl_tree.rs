//! An [`AVLTree`] is a self balancing binary tree

use std::{
    cmp::{self, Ordering},
    iter::{Extend, FromIterator},
    mem,
    ops::Not,
};

/// An internal node of an [`AVLTree`].
#[derive(Debug)]
struct AVLNode<T: Ord> {
    value: T,
    height: usize,
    left: Option<Box<AVLNode<T>>>,
    right: Option<Box<AVLNode<T>>>,
}

impl<T: Ord> AVLNode<T> {
    fn child(&self, side: Side) -> &Option<Box<AVLNode<T>>> {
        match side {
            Side::Left => &self.left,
            Side::Right => &self.right,
        }
    }

    fn child_mut(&mut self, side: Side) -> &mut Option<Box<AVLNode<T>>> {
        match side {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        }
    }

    fn height(&self, side: Side) -> usize {
        self.child(side).as_ref().map_or(0, |n| n.height)
    }

    fn balance_factor(&self) -> i8 {
        let (left, right) = (self.height(Side::Left), self.height(Side::Right));
        if left < right {
            (right - left) as i8
        } else {
            -((left - right) as i8)
        }
    }

    fn update_height(&mut self) {
        self.height = 1 + cmp::max(self.height(Side::Left), self.height(Side::Right));
    }

    fn rotate(&mut self, side: Side) {
        let mut subtree = self.child_mut(!side).take().unwrap();
        *self.child_mut(!side) = subtree.child_mut(side).take();
        self.update_height();
        // Swap root and child nodes in memory
        mem::swap(self, subtree.as_mut());
        // Set old root (subtree) as child of new root (self)
        *self.child_mut(side) = Some(subtree);
        self.update_height();
    }

    fn rebalance(&mut self) {
        self.update_height();
        let side = match self.balance_factor() {
            -2 => Side::Left,
            2 => Side::Right,
            _ => return,
        };
        let subtree = self.child_mut(side).as_mut().unwrap();
        // Left-Right and Right-Left require rotation of heavy subtree
        if let (Side::Left, 1) | (Side::Right, -1) = (side, subtree.balance_factor()) {
            subtree.rotate(side);
        }
        // Rotate in opposite direction of heavy side
        self.rotate(!side);
    }
}

#[derive(Debug, Clone, Copy)]
enum Side {
    Left,
    Right,
}

impl Not for Side {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

/// A set based on an AVL Tree.
///
/// An AVL Tree is a self-balancing binary search tree. It tracks the height of
/// each node and performs internal rotations to maintain a height difference of
/// at most 1 between each sibling pair.
#[derive(Debug)]
pub struct AVLTree<T: Ord> {
    root: Option<Box<AVLNode<T>>>,
    length: usize,
}

impl<T: Ord> Default for AVLTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> FromIterator<T> for AVLTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree = AVLTree::new();
        for value in iter {
            tree.insert(value);
        }
        tree
    }
}

impl<T: Ord> Extend<T> for AVLTree<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for value in iter {
            self.insert(value);
        }
    }
}

impl<T: Ord> AVLTree<T> {
    /// Creates an empty [`AVLTree`]
    pub const fn new() -> Self {
        Self {
            root: None,
            length: 0,
        }
    }

    /// Returns `true` uf the tree contains a value.
    pub fn contains(&self, value: &T) -> bool {
        let mut current = &self.root;
        while let Some(ref node) = current {
            current = match value.cmp(&node.value) {
                Ordering::Equal => return true,
                Ordering::Less => &node.left,
                Ordering::Greater => &node.right,
            }
        }
        false
    }

    /// Adds a value to the tree.
    ///
    /// Returns `true` if the tree did not yet contain the value.
    pub fn insert(&mut self, value: T) -> bool {
        let inserted = insert(&mut self.root, value);
        if inserted {
            self.length += 1;
        }
        inserted
    }

    /// Removes a value from the tree.
    ///
    /// Returns `true` if the tree contained the value.
    pub fn remove(&mut self, value: &T) -> bool {
        let removed = remove(&mut self.root, value);
        if removed {
            self.length -= 1;
        }
        removed
    }

    /// Returns the number of values in the tree.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Returns `true` if the tree contains no values.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn node_iter(&self) -> NodeIter<T> {
        let cap = self.root.as_ref().map_or(0, |n| n.height);
        let mut node_iter = NodeIter {
            stack: Vec::with_capacity(cap),
        };
        // Initialize stack with path to leftmost child
        let mut child = &self.root;
        while let Some(node) = child {
            node_iter.stack.push(node.as_ref());
            child = &node.left;
        }
        node_iter
    }

    /// Returns an iterator that visits the values in the tree in ascending
    /// order.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            node_iter: self.node_iter(),
        }
    }
}

fn insert<T: Ord>(tree: &mut Option<Box<AVLNode<T>>>, value: T) -> bool {
    match tree {
        Some(ref mut node) => {
            let inserted = match value.cmp(&node.value) {
                Ordering::Equal => false,
                Ordering::Less => insert(&mut node.left, value),
                Ordering::Greater => insert(&mut node.right, value),
            };
            if inserted {
                node.rebalance();
            }
            inserted
        }
        None => {
            *tree = Some(Box::new(AVLNode {
                value,
                height: 1,
                left: None,
                right: None,
            }));
            true
        }
    }
}

fn remove<T: Ord>(tree: &mut Option<Box<AVLNode<T>>>, value: &T) -> bool {
    if let Some(node) = tree {
        let removed = match value.cmp(&node.value) {
            Ordering::Less => remove(&mut node.left, value),
            Ordering::Greater => remove(&mut node.right, value),
            Ordering::Equal => {
                *tree = match (node.left.take(), node.right.take()) {
                    (None, None) => None,
                    (Some(b), None) | (None, Some(b)) => Some(b),
                    (Some(left), Some(right)) => Some(merge(left, right)),
                };
                return true;
            }
        };
        if removed {
            node.rebalance();
        }
        removed
    } else {
        false
    }
}

/// Merges two trees and returns the root of the merged tree.
fn merge<T: Ord>(left: Box<AVLNode<T>>, right: Box<AVLNode<T>>) -> Box<AVLNode<T>> {
    let mut op_right = Some(right);
    // Guaranteed not to panic since right has at least one node
    let mut root = take_min(&mut op_right).unwrap();
    root.left = Some(left);
    root.right = op_right;
    root.rebalance();
    root
}

/// Removes the smallest node from the tree, if one exists.
fn take_min<T: Ord>(tree: &mut Option<Box<AVLNode<T>>>) -> Option<Box<AVLNode<T>>> {
    match tree.take() {
        Some(mut node) => match take_min(&mut node.left) {
            Some(small) => {
                node.rebalance();
                *tree = Some(node);
                Some(small)
            }
            None => {
                *tree = node.right.take();
                Some(node)
            }
        },
        None => None,
    }
}

/// An iterator over the nodes of an [`AVLTree`].
///
/// This struct is created by the method [`AVLTree::node_iter()`].
#[derive(Debug)]
struct NodeIter<'a, T: Ord> {
    stack: Vec<&'a AVLNode<T>>,
}

impl<'a, T: Ord> Iterator for NodeIter<'a, T> {
    type Item = &'a AVLNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().map(|node| {
            let mut child = &node.right;
            // loop {
            //     child = if let Some(subtree) = child {
            //         self.stack.push(subtree.as_ref());
            //         &subtree.left
            //     } else {
            //         break;
            //     };
            // }
            while let Some(subtree) = child {
                self.stack.push(subtree.as_ref());
                child = &subtree.left;
            }
            node
        })
    }
}

/// An iterator over the items of an `AVLTree`.
///
/// This struct is created by the `iter` method of `AVLTree`.
#[derive(Debug)]
pub struct Iter<'a, T: Ord> {
    node_iter: NodeIter<'a, T>,
}

impl<'a, T: Ord> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.node_iter.next().map(|node| &node.value)
    }
}

#[cfg(test)]
mod tests {
    use super::AVLTree;

    /// Returns `true` if all nodes in the tree are balanced.
    fn is_balanced<T: Ord>(tree: &AVLTree<T>) -> bool {
        tree.node_iter()
            .all(|n| (-1..=1).contains(&n.balance_factor()))
    }

    #[test]
    fn test_len() {
        let tree: AVLTree<_> = (1..4).collect();
        assert_eq!(tree.len(), 3);
    }

    #[test]
    fn test_contains() {
        let tree: AVLTree<_> = (1..4).collect();
        assert!(tree.contains(&1));
        assert!(!tree.contains(&4));
    }

    #[test]
    fn test_insert() {
        let mut tree = AVLTree::new();
        assert!(tree.insert(1));
        assert!(!tree.insert(1));
    }

    #[test]
    fn test_remove() {
        let mut tree: AVLTree<_> = (1..8).collect();
        // First remove succeeds
        assert!(tree.remove(&4));
        // Second remove fails
        assert!(!tree.remove(&4));
    }

    #[test]
    fn test_sorted() {
        let tree: AVLTree<_> = (1..8).rev().collect();
        assert!((1..8).eq(tree.iter().copied()));
    }

    #[test]
    fn test_balanced() {
        let mut tree: AVLTree<_> = (1..8).collect();
        assert!(is_balanced(&tree));
        for x in 1..8 {
            tree.remove(&x);
            assert!(is_balanced(&tree));
        }
    }
}
