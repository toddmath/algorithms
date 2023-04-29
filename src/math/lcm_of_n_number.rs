//! Calculate the least common multiple of n numbers

use std::ops::{Div, Mul, Rem};

/// Calculate the least common multiple of n numbers
pub fn lcm<T>(nums: &[T]) -> T
where
    T: Copy + Mul<Output = T> + Div<Output = T> + Rem<Output = T> + PartialEq + Into<usize>,
{
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two(a, b)
}

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
        assert_eq!(lcm(&[1usize, 2, 3, 4, 5]), 60);
        assert_eq!(lcm(&[2usize, 4, 6, 8, 10]), 120);
        assert_eq!(lcm(&[3usize, 6, 9, 12, 15]), 180);
        assert_eq!(lcm(&[10usize]), 10);
        assert_eq!(lcm(&[21usize, 110]), 2310);
    }
}
