//! Calculate the least common multiple of n numbers

// use num_integer::Integer;
use std::ops::{BitOr, Div, Mul, Rem};

use num_traits::PrimInt;

/// Calculate the least common multiple of given numbers
pub fn lcm<T: PrimInt + Copy + BitOr<Output = T>>(nums: &[T]) -> T {
    debug_assert!(!nums.is_empty(), "Cannot calculate lcm of an empty array.");
    if nums.len() == 1 {
        return nums[0];
    }
    // let a = nums[0];
    // let b = lcm(&nums[1..]);
    // a * (b / gcd(a, b))
    nums.iter()
        .copied()
        .reduce(|a, b| a * (b / gcd(a, b)))
        .unwrap()
}

fn gcd<T: PrimInt + Copy + BitOr<Output = T>>(a: T, b: T) -> T {
    // Use Stein's algorithm
    if a.is_zero() || b.is_zero() {
        return a | b;
    }

    // find common factors of 2
    let shift = (a | b).trailing_zeros();

    // divide a and b by 2 until odd
    let mut a = a.to_u32().unwrap() >> a.trailing_zeros();
    let mut b = b.to_u32().unwrap() >> b.trailing_zeros();

    while a != b {
        if a > b {
            a -= b;
            a >>= a.trailing_zeros();
        } else {
            b -= a;
            b >>= b.trailing_zeros();
        }
    }
    T::from(a << shift).unwrap()
}

/// Calculate the least common multiple of n numbers
#[allow(dead_code)]
fn lcm2<T>(nums: &[T]) -> T
where
    T: Copy + Mul<Output = T> + Div<Output = T> + Rem<Output = T> + PartialEq + Into<usize>,
{
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm2(&nums[1..]);
    a * b / gcd_of_two(a, b)
}

#[allow(dead_code)]
fn gcd_of_two<T>(a: T, b: T) -> T
where
    T: Copy + Mul<Output = T> + Div<Output = T> + Rem<Output = T> + PartialEq + Into<usize>,
{
    if b.into() == 0 {
        return a;
    }
    gcd_of_two(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(lcm(&[1, 2, 3, 4, 5]), 60);
        assert_eq!(lcm(&[2, 4, 6, 8, 10]), 120);
        assert_eq!(lcm(&[3, 6, 9, 12, 15]), 180);
        assert_eq!(lcm(&[10]), 10);
        assert_eq!(lcm(&[21, 110]), 2310);
    }

    #[test]
    fn empty_array() {
        let result = std::panic::catch_unwind(|| lcm::<u32>(&[]));
        assert!(result.is_err());
    }
}
