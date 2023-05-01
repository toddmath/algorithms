//! Standard implementation of a queue.

use std::collections::LinkedList;

/// A queue is a FIFO (first-in, first-out) data structure.
#[derive(Debug)]
pub struct Queue<T> {
    elements: LinkedList<T>,
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Queue<T> {
    /// Creates a new [`Queue<T>`].
    pub fn new() -> Self {
        Self {
            elements: LinkedList::new(),
        }
    }

    /// Adds an element to the back of the [`Queue<T>`].
    pub fn enqueue(&mut self, value: T) {
        self.elements.push_back(value);
    }

    /// Returns the next item from this [`Queue<T>`].
    pub fn dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    /// Peek the value in the front of this [`Queue<T>`].
    pub fn peek_front(&self) -> Option<&T> {
        self.elements.front()
    }

    /// Returns the length of this [`Queue<T>`].
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns whether this [`Queue<T>`] is empty.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn enqueue() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(64);
        assert!(!queue.is_empty());
    }

    #[test]
    fn dequeue() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(32);
        queue.enqueue(64);
        let retrieved_dequeue = queue.dequeue();
        assert_eq!(retrieved_dequeue, Some(32));
    }

    #[test]
    fn peek_front() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(8);
        queue.enqueue(16);
        let retrieved_peek = queue.peek_front();
        assert_eq!(retrieved_peek, Some(&8));
    }

    #[test]
    fn size() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(8);
        queue.enqueue(16);
        assert_eq!(2, queue.len());
    }
}
