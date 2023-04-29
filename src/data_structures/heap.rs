//! Heap data structure
//!
//! Takes a closure as a comparator to allow for min-heap, max-heap,
//! and works with custom key functions

/// A heap data structure
#[allow(missing_debug_implementations)]
pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: Comparator<T>,
}

type Comparator<T> = fn(&T, &T) -> bool;

impl<T> Heap<T>
where
    T: Default,
{
    /// Creates a new [`Heap`].
    pub fn new(comparator: Comparator<T>) -> Self {
        Self {
            count: 0,
            // Add a default in the first spot to offset indexes
            // for the parent/child math to work out.
            // Vecs have to have all the same type so using Default
            // is a way to add an unused item.
            items: vec![T::default()],
            comparator,
        }
    }

    /// Returns the length of this [`Heap`].
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.count
    }

    /// Is this [`Heap`] empty?
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Add value to [`Heap`]
    pub fn add(&mut self, value: T) {
        self.count += 1;
        self.items.push(value);

        // Heapify up
        let mut idx = self.count;
        while self.parent_idx(idx) > 0 {
            let pdx = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[pdx]) {
                self.items.swap(idx, pdx);
            }
            idx = pdx;
        }
    }

    #[inline(always)]
    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    #[inline(always)]
    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    #[inline(always)]
    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    #[inline(always)]
    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        if self.right_child_idx(idx) > self.count {
            self.left_child_idx(idx)
        } else {
            let ldx = self.left_child_idx(idx);
            let rdx = self.right_child_idx(idx);
            if (self.comparator)(&self.items[ldx], &self.items[rdx]) {
                ldx
            } else {
                rdx
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            return None;
        }

        let next = Some(self.items.swap_remove(1));
        self.count -= 1;

        if self.count > 0 {
            // Heapify down
            let mut idx = 1;
            while self.children_present(idx) {
                let cdx = self.smallest_child_idx(idx);
                if !(self.comparator)(&self.items[idx], &self.items[cdx]) {
                    self.items.swap(idx, cdx);
                }
                idx = cdx;
            }
        }

        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_heap() {
        let mut heap = Heap::<i32>::new_max();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn min_heap() {
        let mut heap = Heap::new_min();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn max_heap() {
        let mut heap = Heap::new_max();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }

    #[derive(Default)]
    struct Point(i32, i32);

    #[test]
    fn key_heap() {
        let mut heap: Heap<Point> = Heap::new(|a, b| a.0 < b.0);
        heap.add(Point(1, 5));
        heap.add(Point(3, 10));
        heap.add(Point(-2, 4));
        assert_eq!(heap.len(), 3);
        assert_eq!(heap.next().unwrap().0, -2);
        assert_eq!(heap.next().unwrap().0, 1);
        heap.add(Point(50, 34));
        assert_eq!(heap.next().unwrap().0, 3);
    }
}
