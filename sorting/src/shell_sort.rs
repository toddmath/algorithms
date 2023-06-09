/// Shell sort
pub fn shell_sort<T: Ord + Copy>(arr: &mut [T]) {
    fn insertion<T: Ord + Copy>(arr: &mut [T], start: usize, gap: usize) {
        for i in (start + gap..arr.len()).step_by(gap) {
            let (current, mut pos) = (arr[i], i);
            while pos >= gap && arr[pos - gap] > current {
                arr[pos] = arr[pos - gap];
                pos -= gap;
            }
            arr[pos] = current;
        }
    }

    for (start, gap) in (0..(arr.len() / 2))
        .rev()
        .map(|g| g / 2)
        .flat_map(|gap| (0..gap).map(move |start| (start, gap)))
    {
        insertion(arr, start, gap);
    }
}

#[cfg(test)]
mod test {
    use super::shell_sort;
    use crate::{have_same_elements, is_sorted};

    #[test]
    fn basic() {
        let mut vec = vec![3, 5, 6, 3, 1, 4];
        let cloned = vec.clone();
        shell_sort(&mut vec);
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }

    #[test]
    fn empty() {
        let mut vec: Vec<i32> = vec![];
        let cloned = vec.clone();
        shell_sort(&mut vec);
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }

    #[test]
    fn reverse() {
        let mut vec = vec![6, 5, 4, 3, 2, 1];
        let cloned = vec.clone();
        shell_sort(&mut vec);
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }

    #[test]
    fn already_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        let cloned = vec.clone();
        shell_sort(&mut vec);
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }
}
