use std::{sync::mpsc, thread, time::Duration};

// use itertools::Itertools;
use num_traits::{NumOps, PrimInt, ToPrimitive};

/// Sleep sort
pub fn sleep_sort<T>(arr: &[T]) -> Vec<T>
where
    T: Ord + PrimInt + NumOps + ToPrimitive + Send + 'static,
{
    let len = arr.len();
    let (tx, rx) = mpsc::channel();

    for &x in arr {
        let tx = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(x.to_u64().unwrap() * 20));
            tx.send(x).expect("transmission thread has panicked");
        });
    }

    let mut sorted = Vec::with_capacity(len);
    for _ in 0..len {
        sorted.push(rx.recv().expect("receiver thread has panicked"));
    }
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let res: Vec<isize> = sleep_sort(&[]);
        assert_eq!(res, &[]);
    }

    #[test]
    fn single_element() {
        let res = sleep_sort(&[1usize]);
        assert_eq!(res, &[1]);
    }

    #[test]
    fn sorted_array() {
        let res = sleep_sort(&[1, 2, 3, 4]);
        assert_eq!(res, &[1, 2, 3, 4]);
    }

    #[test]
    fn unsorted_array() {
        let res = sleep_sort(&[3, 4, 2, 1]);
        assert_eq!(res, &[1, 2, 3, 4]);
    }

    #[test]
    fn odd_number_of_elements() {
        let res = sleep_sort(&[3, 1, 7]);
        assert_eq!(res, &[1, 3, 7]);
    }

    #[test]
    fn repeated_elements() {
        let res = sleep_sort(&[1, 1, 1, 1]);
        assert_eq!(res, &[1, 1, 1, 1]);
    }

    #[test]
    fn random_elements() {
        let res = sleep_sort(&[5, 3, 7, 10, 1, 0, 8]);
        assert_eq!(res, &[0, 1, 3, 5, 7, 8, 10]);
    }
}
