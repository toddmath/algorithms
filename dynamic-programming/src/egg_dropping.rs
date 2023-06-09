//! # Egg Dropping Puzzle
use std::{cmp, fmt};

use itertools::Itertools;
use num_traits::PrimInt;

/// [`egg_drop`] returns the least number of egg droppings required to determine
/// the highest floor from which an egg will not break upon dropping.
///
/// ## Warning
///
/// Assumptions:
/// - `n > 0`
pub fn egg_drop<T: PrimInt + fmt::Display>(eggs: T, floors: T) -> T {
    assert!(!eggs.is_zero() && eggs >= T::one());
    if eggs.is_one() || floors.is_zero() || floors.is_one() {
        return floors;
    }

    let int_width = T::zero().count_zeros();
    let eggs_index = eggs
        .to_usize()
        .unwrap_or_else(|| panic!("Error converting {eggs} to int{int_width}"));
    let floors_index = floors
        .to_usize()
        .unwrap_or_else(|| panic!("Error converting {floors} to int{int_width}"));

    let mut egg_drops = (0..=eggs_index)
        .map(|row| {
            (0..=floors_index)
                .map(|c| {
                    T::from(if row == 0 { 0 } else { c })
                        .expect(format!("Error converting {c} to int{int_width}").as_str())
                })
                .collect_vec()
        })
        .collect_vec();

    for (i, j) in (2..=eggs_index).cartesian_product(2..=floors_index) {
        for k in 1..=j {
            egg_drops[i][j] = cmp::min(
                cmp::min(egg_drops[i][j], T::max_value()),
                cmp::max(egg_drops[i - 1][k - 1], egg_drops[i][j - k]) + T::one(),
            );
        }
    }
    egg_drops[eggs_index][floors_index]
}

#[cfg(test)]
mod tests {
    use super::egg_drop;

    #[test]
    fn zero_floors() {
        assert_eq!(egg_drop(5, 0), 0);
    }

    #[test]
    fn one_egg() {
        assert_eq!(egg_drop(1, 8), 8);
    }

    #[test]
    fn eggs2_floors2() {
        assert_eq!(egg_drop(2, 2), 2);
    }

    #[test]
    fn eggs3_floors5() {
        assert_eq!(egg_drop(3, 5), 3);
    }

    #[test]
    fn eggs2_floors10() {
        assert_eq!(egg_drop(2, 10), 4);
    }

    #[test]
    fn eggs2_floors36() {
        assert_eq!(egg_drop(2, 36), 8);
    }

    #[test]
    fn large_floors() {
        assert_eq!(egg_drop(2, 100), 14);
    }
}
