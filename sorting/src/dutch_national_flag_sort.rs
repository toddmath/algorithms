//! A Rust implementation of the Dutch National Flag sorting algorithm.
//!
//! Reference implementation: https://github.com/TheAlgorithms/Python/blob/master/sorts/dutch_national_flag_sort.py
//! More info: https://en.wikipedia.org/wiki/Dutch_national_flag_problem

/// The three colors of the Dutch Flag 🇳🇱
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub enum Colors {
    /// Red
    Red,
    /// White
    White,
    /// Blue
    Blue,
}

/// A Rust implementation of the [Dutch National Flag](https://en.wikipedia.org/wiki/Dutch_national_flag_problem) sorting algorithm.
pub fn dutch_national_flag_sort(mut sequence: Vec<Colors>) -> Vec<Colors> {
    let len = sequence.len();
    if len <= 1 {
        return sequence;
    }
    let mut low = 0;
    let mut mid = 0;
    let mut high = len - 1;

    while mid <= high {
        match sequence[mid] {
            Colors::Red => {
                sequence.swap(low, mid);
                low += 1;
                mid += 1;
            }
            Colors::White => {
                mid += 1;
            }
            Colors::Blue => {
                sequence.swap(mid, high);
                high -= 1;
            }
        }
    }
    sequence
}

#[cfg(test)]
mod tests {
    use super::{dutch_national_flag_sort, Colors::*};
    use crate::is_sorted;

    #[test]
    fn random_array() {
        let arr = vec![
            Red, Blue, White, White, Blue, Blue, Red, Red, White, Blue, White, Red, White, Blue,
        ];
        let arr = dutch_national_flag_sort(arr);
        assert!(is_sorted(&arr))
    }

    #[test]
    fn sorted_array() {
        let arr = vec![
            Red, Red, Red, Red, Red, White, White, White, White, White, Blue, Blue, Blue, Blue,
        ];
        let arr = dutch_national_flag_sort(arr);
        assert!(is_sorted(&arr))
    }
}
