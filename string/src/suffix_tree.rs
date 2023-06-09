//! In computer science, a suffix tree (also called PAT tree or, in an earlier
//! form, position tree) is a compressed trie containing all the suffixes of the
//! given text as their keys and positions in the text as their values. Suffix
//! trees allow particularly fast implementations of many important string operations. [Source](https://en.wikipedia.org/wiki/Suffix_tree)

// use itertools::Itertools;
// use smallvec::SmallVec;

/// [`SuffixTree`] node data structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    /// Substring of input string.
    pub sub: String,
    /// Child nodes.
    pub ch: Vec<usize>,
}

impl Default for Node {
    #[inline]
    fn default() -> Self {
        Self {
            sub: "".to_string(),
            ch: vec![],
        }
    }
}

impl Node {
    /// Create a new node.
    #[inline]
    pub fn new(sub: impl AsRef<str>, children: Vec<usize>) -> Self {
        Self {
            sub: sub.as_ref().to_string(),
            ch: children,
        }
    }

    /// Create a new node with `sub` string.
    #[inline]
    pub fn with_sub(sub: impl AsRef<str>) -> Self {
        Self {
            sub: sub.as_ref().to_string(),
            ch: vec![],
        }
    }

    /// Create a new node with `children` vector.
    #[inline]
    pub fn with_children(children: Vec<usize>) -> Self {
        Self {
            sub: "".to_string(),
            ch: children,
        }
    }

    /// Creates a new node with an empty string and empty vector of children.
    #[inline]
    pub fn empty() -> Self {
        Self::default()
    }
}

/// A [`SuffixTree`] (also called PAT tree or, in an earlier form, position
/// tree) is a compressed trie containing all the suffixes of the given text as
/// their keys and positions in the text as their values. Suffix trees allow
/// particularly fast implementations of many important string operations. [Wikipedia](https://en.wikipedia.org/wiki/Suffix_tree)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SuffixTree {
    /// Nodes of the suffix tree.
    pub nodes: Vec<Node>,
}

impl SuffixTree {
    /// Creates a new [`SuffixTree`].
    pub fn new(s: impl AsRef<str>) -> Self {
        let s = s.as_ref();
        let mut tree = SuffixTree {
            nodes: vec![Node::empty()],
        };
        for i in 0..s.len() {
            tree.add_suffix(s.get(i..).unwrap_or_default());
        }
        tree
    }

    fn add_suffix(&mut self, suffix: impl AsRef<str>) {
        let (mut n, mut i) = (0, 0);
        let suffix = suffix.as_ref();

        while i < suffix.len() {
            let b = suffix.chars().nth(i);
            let mut x2 = 0;
            let mut n2: usize;

            loop {
                let children = &self.nodes[n].ch;
                if children.len() == x2 {
                    n2 = self.nodes.len();
                    self.nodes
                        .push(Node::with_sub(suffix.get(i..).unwrap_or_default()));
                    self.nodes[n].ch.push(n2);
                    return;
                }

                n2 = children[x2];
                if self.nodes[n2].sub.chars().next() == b {
                    break;
                }
                x2 += 1;
            }

            let sub2 = self.nodes[n2].sub.clone();
            let mut j = 0;

            i += loop {
                if j >= sub2.len() {
                    break j;
                }
                if suffix.chars().nth(i + j) != sub2.chars().nth(j) {
                    let n3 = n2;
                    n2 = self.nodes.len();
                    let (sj1, sj2) = sub2.split_at(j);
                    self.nodes.push(Node::new(sj1, vec![n3]));
                    self.nodes[n3].sub = sj2.to_string();
                    self.nodes[n].ch[x2] = n2;
                    break j;
                }
                j += 1;
            };
            n = n2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_tree() {
        let suf_tree = SuffixTree::new("banana$");
        assert_eq!(suf_tree.nodes, vec![
            Node::with_children(vec![1, 8, 6, 10]),
            Node::with_sub("banana$"),
            Node::with_sub("na$"),
            Node::with_sub("na$"),
            Node::new("na", vec![2, 5]),
            Node::with_sub("$"),
            Node::new("na", vec![3, 7]),
            Node::with_sub("$"),
            Node::new("a", vec![4, 9]),
            Node::with_sub("$"),
            Node::with_sub("$"),
        ]);
    }
}
