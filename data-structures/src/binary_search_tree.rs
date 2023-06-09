use std::cmp::Ordering;

// pub struct BST<K: Ord>(Option<Box<BinarySearchTree<T>>>);

// type SubTree<T> = Option<Box<BinarySearchTree<T>>>;

/// This struct implements as Binary Search Tree (BST), which is a
/// simple data structure for storing sorted data
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct BinarySearchTree<T: Ord> {
    value: Option<T>,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}

impl<T: Ord> BinarySearchTree<T> {
    /// Creates a new [`BinarySearchTree<T>`].
    pub const fn new() -> Self {
        Self {
            value: None,
            left: None,
            right: None,
        }
    }

    /// Creates a new [`BinarySearchTree<T>`] with `value` as the root.
    pub fn from(value: T) -> Self {
        Self {
            value: Some(value),
            left: None,
            right: None,
        }
    }

    /// Find a value in this tree. Returns True if value is in this
    /// tree, and false otherwise
    pub fn search(&self, value: &T) -> bool {
        // self.value
        //     .as_ref()
        //     .map_or(false, |key| match key.cmp(value) {
        //         Ordering::Equal => true,
        //         Ordering::Greater => self.left.as_ref().map_or(false, |node|
        // node.search(value)),         Ordering::Less =>
        // self.right.as_ref().map_or(false, |node| node.search(value)),     })
        match &self.value {
            Some(key) => match key.cmp(value) {
                Ordering::Equal => true,
                Ordering::Greater => match &self.left {
                    Some(node) => node.search(value),
                    None => false,
                },
                Ordering::Less => match &self.right {
                    Some(node) => node.search(value),
                    None => false,
                }
                // Ordering::Greater => self.left.as_ref().map_or(false, |node| node.search(value)),
                // Ordering::Less => self.right.as_ref().map_or(false, |node| node.search(value)),
            },
            None => false,
        }
    }

    /// Insert `value` into the appropriate location in this tree.
    pub fn insert(&mut self, value: T)
    where
        T: Copy,
    {
        if let Some(key) = self.value.or(Some(value)) {
            let target_node = if value < key {
                &mut self.left
            } else {
                &mut self.right
            };
            match target_node {
                Some(ref mut node) => node.insert(value),
                None => {
                    let mut node = BinarySearchTree::new();
                    node.insert(value);
                    // target_node.replace(Box::new(node));
                    *target_node = Some(Box::new(node));
                }
            }
        }
    }

    /// Returns the smallest value in this [`BinarySearchTree<T>`].
    pub fn minimum(&self) -> Option<&T> {
        match &self.left {
            Some(node) => node.minimum(),
            None => self.value.as_ref(),
        }
        // self.left
        //     .as_ref()
        //     .map_or(self.value.as_ref(), |node| node.minimum())
    }

    /// Returns the largest value in this [`BinarySearchTree<T>`].
    pub fn maximum(&self) -> Option<&T> {
        match &self.right {
            Some(node) => node.maximum(),
            None => self.value.as_ref(),
        }
        // self.right
        //     .as_ref()
        //     .map_or(self.value.as_ref(), |node| node.maximum())
    }

    /// Returns the largest `value` in this [`BinarySearchTree<T>`] smaller than
    /// `value`.
    pub fn floor(&self, value: &T) -> Option<&T> {
        // self.value.as_ref().and_then(|key| match key.cmp(value) {
        if let Some(ref key) = self.value {
            match key.cmp(value) {
                Ordering::Equal => Some(key),
                Ordering::Greater => self.left.as_ref().and_then(|node| node.floor(value)),
                Ordering::Less => match self.right.as_ref() {
                    Some(node) => {
                        let val = node.floor(value);
                        match val {
                            Some(_) => val,
                            None => Some(key),
                        }
                    }
                    None => Some(key),
                },
            }
        } else {
            None
        }
        // match &self.value {
        //     Some(key) => match key.cmp(value) {
        //         Ordering::Equal => Some(key),
        //         Ordering::Greater => self.left.as_ref().and_then(|node|
        // node.floor(value)),         // Ordering::Greater => match
        // &self.left {         //     Some(node) => node.floor(value),
        //         //     None => None,
        //         // }
        //         Ordering::Less => match &self.right {
        //             Some(node) => {
        //                 let val = node.floor(value);
        //                 val.or(Some(key))
        //                 // match val {
        //                 //     Some(_) => val,
        //                 //     None => Some(key),
        //                 // }
        //             }
        //             None => Some(key),
        //         }
        //     // Ordering::Greater => self.left.as_ref().and_then(|node|
        // node.floor(value)),     // Ordering::Less => self
        //     //     .right
        //     //     .as_ref()
        //     //     .map_or(Some(key), |node|
        // node.floor(value).or(Some(key))),     },
        //     None => None,
        // }
    }

    /// Returns the smallest `value` in this [`BinarySearchTree<T>`] larger than
    /// `value`.
    pub fn ceil(&self, value: &T) -> Option<&T> {
        // self.value.as_ref().and_then(|key| match key.cmp(value) {
        match &self.value {
            Some(key) => match key.cmp(value) {
                Ordering::Equal => Some(key),
                Ordering::Less => self.right.as_ref().and_then(|node| node.ceil(value)),
                Ordering::Greater => match &self.left {
                    Some(node) => {
                        let val = node.ceil(value);
                        match val {
                            Some(_) => val,
                            None => Some(key)
                        }
                    }
                    None => Some(key),
                }
                // Ordering::Less => match &self.right {
                //     Some(node) => node.ceil(value),
                //     None => None,
                // },
                // Ordering::Less => self.right.as_ref().and_then(|node| node.ceil(value)),
                // Ordering::Greater => self
                //     .left
                //     .as_ref()
                //     .map_or(Some(key), |node| node.ceil(value).or(Some(key))),
            },
            None => None,
        }
    }

    /// Returns a new iterator which iterates, in order, over this tree.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        BinarySearchTreeIter::new(self)
    }
}

impl<T: Ord> Drop for BinarySearchTree<T> {
    fn drop(&mut self) {
        // for _child in self.iter() {}
        drop(self.iter())
    }
}

struct BinarySearchTreeIter<'a, T: Ord> {
    stack: Vec<&'a BinarySearchTree<T>>,
}

impl<'a, T: Ord> BinarySearchTreeIter<'a, T> {
    pub fn new(tree: &'a BinarySearchTree<T>) -> Self {
        let stack = vec![tree];
        let mut iter = BinarySearchTreeIter { stack };
        iter.stack_push_left();
        iter
    }

    fn stack_push_left(&mut self) {
        // while let Some(child) = &self.stack.last().unwrap().left {
        //     self.stack.push(child);
        // }
        while let Some(child) = self.stack.last() {
            if let Some(ref left) = child.left {
                self.stack.push(left);
            } else {
                break;
            }
        }
    }
}

impl<'a, T: Ord> Iterator for BinarySearchTreeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            return None;
        }
        self.stack.pop().and_then(|node| {
            if let Some(ref right) = node.right {
                self.stack.push(right);
                self.stack_push_left();
            }
            node.value.as_ref()
        })
        // if let Some(node) = self.stack.pop() {
        //     if let Some(ref right) = node.right {
        //         self.stack.push(right);
        //         self.stack_push_left();
        //     }
        //     // let node = self.stack.pop()?;
        //     // if node.right.is_some() {
        //     //     self.stack.push(node.right.as_ref()?);
        //     //     self.stack_push_left();
        //     // }
        //     node.value.as_ref()
        // } else {
        //     None
        // }
    }
}

#[cfg(test)]
mod test {
    use super::BinarySearchTree;

    fn prequel_memes_tree() -> BinarySearchTree<&'static str> {
        let mut tree = BinarySearchTree::new();
        tree.insert("hello there");
        tree.insert("general kenobi");
        tree.insert("you are a bold one");
        tree.insert("kill him");
        tree.insert("back away...I will deal with this jedi slime myself");
        tree.insert("your move");
        tree.insert("you fool");
        tree
    }

    #[test]
    fn test_search() {
        let tree = prequel_memes_tree();
        assert!(tree.search(&"hello there"));
        assert!(tree.search(&"you are a bold one"));
        assert!(tree.search(&"general kenobi"));
        assert!(tree.search(&"you fool"));
        assert!(tree.search(&"kill him"));
        assert!(
            !tree.search(&"but i was going to tosche station to pick up some power converters",)
        );
        assert!(!tree.search(&"only a sith deals in absolutes"));
        assert!(!tree.search(&"you underestimate my power"));
    }

    #[test]
    fn test_maximum_and_minimum() {
        let tree = prequel_memes_tree();
        assert_eq!(*tree.maximum().unwrap(), "your move");
        assert_eq!(
            *tree.minimum().unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        let mut tree2: BinarySearchTree<i32> = BinarySearchTree::new();
        assert!(tree2.maximum().is_none());
        assert!(tree2.minimum().is_none());
        tree2.insert(0);
        assert_eq!(*tree2.minimum().unwrap(), 0);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(-5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 5);
    }

    #[test]
    fn test_floor_and_ceil() {
        let tree = prequel_memes_tree();
        assert_eq!(*tree.floor(&"hello there").unwrap(), "hello there");
        assert_eq!(
            *tree
                .floor(&"these are not the droids you're looking for")
                .unwrap(),
            "kill him"
        );
        assert!(tree.floor(&"another death star").is_none());
        assert_eq!(*tree.floor(&"you fool").unwrap(), "you fool");
        assert_eq!(
            *tree.floor(&"but i was going to tasche station").unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        assert_eq!(
            *tree.floor(&"you underestimate my power").unwrap(),
            "you fool"
        );
        assert_eq!(*tree.floor(&"your new empire").unwrap(), "your move");
        assert_eq!(*tree.ceil(&"hello there").unwrap(), "hello there");
        assert_eq!(
            *tree
                .ceil(&"these are not the droids you're looking for")
                .unwrap(),
            "you are a bold one"
        );
        assert_eq!(
            *tree.ceil(&"another death star").unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        assert_eq!(*tree.ceil(&"you fool").unwrap(), "you fool");
        assert_eq!(
            *tree.ceil(&"but i was going to tasche station").unwrap(),
            "general kenobi"
        );
        assert_eq!(
            *tree.ceil(&"you underestimate my power").unwrap(),
            "your move"
        );
        assert!(tree.ceil(&"your new empire").is_none());
    }

    #[test]
    fn test_iterator() {
        let tree = prequel_memes_tree();
        let mut iter = tree.iter();
        assert_eq!(
            iter.next().unwrap(),
            &"back away...I will deal with this jedi slime myself"
        );
        assert_eq!(iter.next().unwrap(), &"general kenobi");
        assert_eq!(iter.next().unwrap(), &"hello there");
        assert_eq!(iter.next().unwrap(), &"kill him");
        assert_eq!(iter.next().unwrap(), &"you are a bold one");
        assert_eq!(iter.next().unwrap(), &"you fool");
        assert_eq!(iter.next().unwrap(), &"your move");
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
