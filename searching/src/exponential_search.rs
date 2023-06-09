use std::cmp::{self, Ordering};

/// Exponential Search
pub fn exponential_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }
    let mut upper = 1;
    while upper < arr.len() && &arr[upper] <= item {
        upper *= 2;
    }

    upper = cmp::min(upper, arr.len());
    let mut lower = upper / 2;

    while lower < upper {
        let mid = lower + (upper - lower) / 2;

        match item.cmp(&arr[mid]) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => upper = mid,
            Ordering::Greater => lower = mid + 1,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let index = exponential_search(&"a", &[]);
        assert_eq!(index, None);
    }

    #[test]
    fn one_item() {
        let index = exponential_search(&"a", &["a"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_strings() {
        let index = exponential_search(&"a", &["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_ints() {
        let index = exponential_search(&4, &[1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = exponential_search(&3, &[1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = exponential_search(&2, &[1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = exponential_search(&1, &[1, 2, 3, 4]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn not_found() {
        let index = exponential_search(&5, &[1, 2, 3, 4]);
        assert_eq!(index, None);
    }
}
