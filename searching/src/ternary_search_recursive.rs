use std::cmp::Ordering;

/// Ternary search recursive.
pub fn ternary_search_rec<T: Ord>(
    arr: &[T],
    target: &T,
    start: usize,
    end: usize,
) -> Option<usize> {
    if arr.is_empty() || start > end {
        return None;
    }
    let (mid1, mid2) = (start + (end - start) / 3, end - (end - start) / 3);

    match target.cmp(&arr[mid1]) {
        Ordering::Equal => Some(mid1),
        Ordering::Less => ternary_search_rec(arr, target, start, mid1 - 1),
        Ordering::Greater => match target.cmp(&arr[mid2]) {
            Ordering::Equal => Some(mid2),
            Ordering::Less => ternary_search_rec(arr, target, mid1 + 1, mid2 - 1),
            Ordering::Greater => ternary_search_rec(arr, target, mid2 + 1, end),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_none_if_empty_list() {
        let index = ternary_search_rec(&[], &"a", 1, 10);
        assert_eq!(index, None);
    }

    #[test]
    fn returns_none_if_range_is_invalid() {
        let index = ternary_search_rec(&[1, 2, 3], &1, 2, 1);
        assert_eq!(index, None);
    }

    #[test]
    fn returns_index_if_list_has_one_item() {
        let index = ternary_search_rec(&[1], &1, 0, 1);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn returns_first_index() {
        let index = ternary_search_rec(&[1, 2, 3], &1, 0, 2);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn returns_first_index_if_end_out_of_bounds() {
        let index = ternary_search_rec(&[1, 2, 3], &1, 0, 3);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn returns_last_index() {
        let index = ternary_search_rec(&[1, 2, 3], &3, 0, 2);
        assert_eq!(index, Some(2));
    }

    #[test]
    fn returns_last_index_if_end_out_of_bounds() {
        let index = ternary_search_rec(&[1, 2, 3], &3, 0, 3);
        assert_eq!(index, Some(2));
    }

    #[test]
    fn returns_middle_index() {
        let index = ternary_search_rec(&[1, 2, 3], &2, 0, 2);
        assert_eq!(index, Some(1));
    }

    #[test]
    fn returns_middle_index_if_end_out_of_bounds() {
        let index = ternary_search_rec(&[1, 2, 3], &2, 0, 3);
        assert_eq!(index, Some(1));
    }
}
