use std::cmp;

/// Jump Search
pub fn jump_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }
    let get_step = |x: usize| (x as f64).sqrt() as usize;
    let len = arr.len();
    let mut step = get_step(len);
    let mut prev = 0;

    while arr[cmp::min(len, step) - 1] < *item {
        prev = step;
        step += get_step(len);
        if prev >= len {
            return None;
        }
    }
    while &arr[prev] < item {
        prev += 1;
        if prev == cmp::min(step, len) {
            return None;
        }
    }

    if arr[prev] == *item { Some(prev) } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let index = jump_search(&"a", &[]);
        assert_eq!(index, None);
    }

    #[test]
    fn one_item() {
        let index = jump_search(&"a", &["a"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_strings() {
        let index = jump_search(&"a", &["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_ints() {
        let index = jump_search(&4, &[1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = jump_search(&3, &[1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = jump_search(&2, &[1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = jump_search(&1, &[1, 2, 3, 4]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn not_found() {
        let index = jump_search(&5, &[1, 2, 3, 4]);
        assert_eq!(index, None);
    }
}
