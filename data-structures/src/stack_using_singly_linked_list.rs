//! Stack data structure

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
/// Simple stack data structure
pub struct Stack<T> {
    head: Link<T>,
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Default for Stack<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Stack<T> {
    /// Creates a new [`Stack<T>`].
    pub fn new() -> Self {
        Self { head: None }
    }

    /// Add element to end of [`Stack<T>`]
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head.replace(new_node);
    }

    /// Pop element from the [`Stack<T>`]
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
        // match self.head.take() {
        //     None => Err("Stack is empty"),
        //     Some(node) => {
        //         self.head = node.next;
        //         Ok(node.elem)
        //     }
        // }
    }

    #[inline(always)]
    /// Is this [`Stack<T>`] empty?
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    #[inline]
    /// Peek the next value in this [`Stack<T>`].
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    #[inline]
    /// Peek the next value in this [`Stack<T>`].
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    /// Returns an iterator for this [`Stack<T>`].
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    /// Returns a mutable iterator for this [`Stack<T>`].
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(ref mut node) = cur_link {
            cur_link = node.next.take();
        }
    }
}

impl<T> Extend<T> for Stack<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for elem in iter {
            self.push(elem);
        }
    }
}

impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut stack = Stack::default();
        for elem in iter {
            stack.push(elem);
        }
        stack
    }
}

impl<T> IntoIterator for Stack<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

#[derive(Debug)]
pub struct IntoIter<T>(Stack<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
        // self.0.pop().ok()
    }
}

#[derive(Debug)]
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

#[derive(Debug)]
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut list = Stack::new();
        assert!(list.pop().is_none());
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        list.push(4);
        list.push(5);
        assert!(!list.is_empty());
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert!(list.pop().is_none());
        assert!(list.is_empty());
    }

    #[test]
    fn peek() {
        let mut list = Stack::new();
        assert!(list.peek().is_none());
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        if let Some(value) = list.peek_mut() {
            *value = 42
        };

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }

    #[test]
    fn extend() {
        let mut list = Stack::new();
        list.extend(vec![1, 2, 3]);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert!(list.pop().is_none() && list.is_empty());

        list.extend(vec![3, 2, 1]);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert!(list.pop().is_none() && list.is_empty());
    }

    #[test]
    fn from_iter() {
        let mut list = Stack::from_iter(vec![1, 2, 3]);
        assert!(!list.is_empty());
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert!(list.pop().is_none() && list.is_empty());
    }
}
