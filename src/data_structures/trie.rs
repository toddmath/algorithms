//! Trie data structure

use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Default)]
struct Node<Key: Default, Type: Default> {
    children: HashMap<Key, Node<Key, Type>>,
    value: Option<Type>,
}

/// Trie data structure
#[derive(Debug, Default)]
pub struct Trie<Key, Type>
where
    Key: Default + Eq + Hash,
    Type: Default,
{
    root: Node<Key, Type>,
}

impl<Key, Type> Trie<Key, Type>
where
    Key: Default + Eq + Hash,
    Type: Default,
{
    /// Creates a new [`Trie`].
    pub fn new() -> Self {
        Self {
            root: Node::default(),
        }
    }

    /// Insert a value into the trie
    pub fn insert(&mut self, key: impl IntoIterator<Item = Key>, value: Type)
    where
        Key: Eq + Hash,
    {
        let mut node = &mut self.root;
        for c in key.into_iter() {
            node = node.children.entry(c).or_insert_with(Node::default);
        }
        node.value.replace(value);
        // node.value = Some(value);
    }

    /// Get a value from the trie
    pub fn get(&self, key: impl IntoIterator<Item = Key>) -> Option<&Type>
    where
        Key: Eq + Hash,
    {
        let mut node = &self.root;
        for c in key.into_iter() {
            if node.children.contains_key(&c) {
                node = node.children.get(&c).unwrap()
            } else {
                return None;
            }
        }
        node.value.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertion() {
        let mut trie = Trie::new();
        assert_eq!(trie.get("".chars()), None);

        trie.insert("foo".chars(), 1);
        trie.insert("foobar".chars(), 2);

        let mut trie = Trie::new();
        assert_eq!(trie.get(vec![1, 2, 3]), None);

        trie.insert(vec![1, 2, 3], 1);
        trie.insert(vec![3, 4, 5], 2);
    }

    #[test]
    fn get() {
        let mut trie = Trie::new();
        trie.insert("foo".chars(), 1);
        trie.insert("foobar".chars(), 2);
        trie.insert("bar".chars(), 3);
        trie.insert("baz".chars(), 4);

        assert_eq!(trie.get("foo".chars()), Some(&1));
        assert_eq!(trie.get("food".chars()), None);

        let mut trie = Trie::new();
        trie.insert(vec![1, 2, 3, 4], 1);
        trie.insert(vec![42], 2);
        trie.insert(vec![42, 6, 1000], 3);
        trie.insert(vec![1, 2, 4, 16, 32], 4);

        assert_eq!(trie.get(vec![42, 6, 1000]), Some(&3));
        assert_eq!(trie.get(vec![43, 44, 45]), None);
    }
}
