use data_structures::Heap;

/// Returns k-th smallest element of an array.
/// Time complexity is stably O(nlog(k)) in all cases
/// Extra space is required to maintain the heap, and it doesn't
/// mutate the input list.
///
/// It is preferrable to the partition-based algorithm in cases when
/// we want to maintain the kth smallest element dynamically against
/// a stream of elements. In that case, once the heap is built, further
/// operation's complexity is O(log(k)).
pub fn kth_smallest_heap<T: Ord + Copy + Default>(input: &[T], k: usize) -> Option<T> {
    if input.len() < k {
        return None;
    }
    let mut heap = Heap::new_max();
    for &val in input.iter().take(k) {
        heap.add(val);
    }

    for &val in input.iter().skip(k) {
        let cur_big = heap.next().unwrap();
        heap.add(if val > cur_big { cur_big } else { val });
    }

    heap.next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let zero: [u8; 0] = [];
        let first = kth_smallest_heap(&zero, 1);

        assert_eq!(None, first);
    }

    #[test]
    fn one_element() {
        let one = [1];
        let first = kth_smallest_heap(&one, 1);

        assert_eq!(1, first.unwrap());
    }

    #[test]
    fn many_elements() {
        // 0 1 3 4 5 7 8 9 9 10 12 13 16 17
        let many = [9, 17, 3, 16, 13, 10, 1, 5, 7, 12, 4, 8, 9, 0];

        let first = kth_smallest_heap(&many, 1);
        let third = kth_smallest_heap(&many, 3);
        let sixth = kth_smallest_heap(&many, 6);
        let fourteenth = kth_smallest_heap(&many, 14);

        assert_eq!(0, first.unwrap());
        assert_eq!(3, third.unwrap());
        assert_eq!(7, sixth.unwrap());
        assert_eq!(17, fourteenth.unwrap());
    }
}
