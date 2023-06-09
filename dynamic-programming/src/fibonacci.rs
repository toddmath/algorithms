//! Fibonacci Sequence via Dynamic Programming

use std::{collections::HashMap, hash::Hash};

use num_integer::Integer;
use num_traits::PrimInt;

/// [`fibonacci`] returns the `n`th fibonacci number
///
/// This function uses the definition of Fibonacci where:
/// `F(0) = F(1) = 1` and `F(n+1) = F(n) + F(n-1)` for `n > 0`
///
/// Warning: This will overflow the 128-bit unsigned integer at n=186
pub fn fibonacci<T: PrimInt>(n: T) -> u128 {
    let (mut a, mut b) = (0, 1);
    for _ in 0..n.to_i32().unwrap() {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

/// [`recursive_fibonacci`] returns the `n`th fibonacci number
///
/// This function uses the definition of Fibonacci where:
/// `F(0) = F(1) = 1` and `F(n+1) = F(n) + F(n-1)` for `n>0`
///
/// Warning: This will overflow the 128-bit unsigned integer at n=186
pub fn recursive_fibonacci<T: PrimInt>(n: T) -> u128 {
    fn fibonacci<T: PrimInt>(n: T, prev: u128, current: u128) -> u128 {
        if n.is_zero() {
            current
        } else {
            fibonacci(n - T::one(), current, current + prev)
        }
    }
    fibonacci(n, 0, 1)
}

/// [`classical_fibonacci`] returns the `n`th fibonacci number
///
/// This function uses the definition of Fibonacci where:
/// `F(0) = F(1) = 1` and `F(n+1) = F(n) + F(n-1)` for `n>0`
///
/// Warning: This will overflow the 128-bit unsigned integer at n=186
pub fn classical_fibonacci<T: PrimInt>(n: T) -> u128 {
    if n.is_zero() || n.is_one() {
        return n.to_u128().unwrap();
    }
    let k = n / T::from(2).unwrap();
    let f1 = classical_fibonacci(k);
    let f2 = classical_fibonacci(k - T::one());
    let x = n % T::from(4).unwrap();

    if x.is_zero() || x == T::from(2).unwrap() {
        f1 * (f1 + 2 * f2)
    } else if x.is_one() {
        (2 * f1 + f2) * (2 * f1 - f2) + 2
    } else {
        (2 * f1 + f2) * (2 * f1 - f2) - 2
    }
}

/// [`logarithmic_fibonacci`] returns the `n`th fibonacci number
///
/// This function uses the definition of Fibonacci where:
/// `F(0) = F(1) = 1` and `F(n+1) = F(n) + F(n-1)` for `n>0`
///
/// Warning: This will overflow the 128-bit unsigned integer at n=186
pub fn logarithmic_fibonacci<T: PrimInt + Integer>(n: T) -> u128 {
    fn fibonacci<T: PrimInt + Integer>(n: T) -> (u128, u128) {
        if n.is_zero() {
            return (0, 1);
        }
        let (current, next) = fibonacci(n / T::from(2).unwrap());
        let c = current * (next * 2 - current);
        let d = current * current + next * next;
        if n.is_even() { (c, d) } else { (d, c + d) }
    }
    if n == T::from(186).unwrap() {
        let (_, second) = fibonacci(185);
        return second;
    }
    let (first, _) = fibonacci(n);
    first
}

/// Memoized fibonacci
pub fn memoized_fibonacci<T: PrimInt + Integer + Hash>(n: T) -> u128 {
    let mut cache = HashMap::new();

    fn fibonacci<T: PrimInt + Integer + Hash>(n: T, cache: &mut HashMap<T, u128>) -> u128 {
        if n.is_zero() || n.is_one() {
            return n.to_u128().unwrap();
        }
        let f = match cache.get(&n) {
            Some(f) => f,
            None => {
                let res =
                    fibonacci(n - T::one(), cache) + fibonacci(n - T::from(2).unwrap(), cache);
                cache.insert(n, res);
                cache.get(&n).unwrap()
            }
        };
        *f
    }

    fibonacci(n, &mut cache)
}

/// [`matrix_fibonacci`] returns the `n`th fibonacci number
///
/// This function uses the definition of Fibonacci where:
/// `F(0) = 0, F(1) = 1` and `F(n+1) = F(n) + F(n-1)` for `n>0`
///
/// Matrix formula:
/// ```text
/// [F(n + 2)]  =  [1, 1] * [F(n + 1)]
/// [F(n + 1)]     [1, 0]   [F(n)    ]
/// ```
///
/// Warning: This will overflow the 128-bit unsigned integer at n=186
pub fn matrix_fibonacci<T: PrimInt + Integer>(n: T) -> u128 {
    let multiplier = matrix_power(&vec![vec![1u128, 1], vec![1, 0]], n);
    let multiplicand = vec![vec![1u128], vec![0]];
    let res = matrix_multiply(&multiplier, &multiplicand);
    res[1][0]
}

fn matrix_power<T: PrimInt + Integer>(base: &Vec<Vec<u128>>, power: T) -> Vec<Vec<u128>> {
    let identity_matrix = vec![vec![1u128, 0], vec![0, 1]];
    vec![base; power.to_usize().unwrap()]
        .into_iter()
        .fold(identity_matrix, |acc, x| matrix_multiply(&acc, x))
}

fn matrix_multiply(multiplier: &[Vec<u128>], multiplicand: &[Vec<u128>]) -> Vec<Vec<u128>> {
    let mut result = vec![];
    let mut temp;
    let row_right_len = multiplicand[0].len();

    for row_left in 0..multiplicand.len() {
        assert_eq!(
            multiplier[row_left].len(),
            multiplicand.len(),
            "Matrix dimensions do not match"
        );
        result.push(vec![]);

        for col_right in 0..multiplicand[0].len() {
            temp = 0;
            #[allow(clippy::needless_range_loop)]
            for row_right in 0..multiplicand.len() {
                assert_eq!(
                    row_right_len,
                    multiplicand[row_right].len(),
                    "Matrix dimensions do not match"
                );
                temp += multiplier[row_left][row_right] * multiplicand[row_right][col_right];
            }
            result[row_left].push(temp);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 1);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 2);
        assert_eq!(fibonacci(3), 3);
        assert_eq!(fibonacci(4), 5);
        assert_eq!(fibonacci(5), 8);
        assert_eq!(fibonacci(10), 89);
        assert_eq!(fibonacci(20), 10946);
        assert_eq!(fibonacci(100), 573147844013817084101);
        assert_eq!(fibonacci(184), 205697230343233228174223751303346572685);
    }

    #[test]
    fn test_recursive_fibonacci() {
        assert_eq!(recursive_fibonacci(0), 1);
        assert_eq!(recursive_fibonacci(1), 1);
        assert_eq!(recursive_fibonacci(2), 2);
        assert_eq!(recursive_fibonacci(3), 3);
        assert_eq!(recursive_fibonacci(4), 5);
        assert_eq!(recursive_fibonacci(5), 8);
        assert_eq!(recursive_fibonacci(10), 89);
        assert_eq!(recursive_fibonacci(20), 10946);
        assert_eq!(recursive_fibonacci(100), 573147844013817084101);
        assert_eq!(
            recursive_fibonacci(184),
            205697230343233228174223751303346572685
        );
    }

    #[test]
    fn test_classical_fibonacci() {
        assert_eq!(classical_fibonacci(0), 0);
        assert_eq!(classical_fibonacci(1), 1);
        assert_eq!(classical_fibonacci(2), 1);
        assert_eq!(classical_fibonacci(3), 2);
        assert_eq!(classical_fibonacci(4), 3);
        assert_eq!(classical_fibonacci(5), 5);
        assert_eq!(classical_fibonacci(10), 55);
        assert_eq!(classical_fibonacci(20), 6765);
        assert_eq!(classical_fibonacci(21), 10946);
        assert_eq!(classical_fibonacci(100), 354224848179261915075);
        assert_eq!(
            classical_fibonacci(184),
            127127879743834334146972278486287885163
        );
    }

    #[test]
    fn test_logarithmic_fibonacci() {
        assert_eq!(logarithmic_fibonacci(0), 0);
        assert_eq!(logarithmic_fibonacci(1), 1);
        assert_eq!(logarithmic_fibonacci(2), 1);
        assert_eq!(logarithmic_fibonacci(3), 2);
        assert_eq!(logarithmic_fibonacci(4), 3);
        assert_eq!(logarithmic_fibonacci(5), 5);
        assert_eq!(logarithmic_fibonacci(10), 55);
        assert_eq!(logarithmic_fibonacci(20), 6765);
        assert_eq!(logarithmic_fibonacci(21), 10946);
        assert_eq!(logarithmic_fibonacci(100), 354224848179261915075);
        assert_eq!(
            logarithmic_fibonacci(184),
            127127879743834334146972278486287885163
        );
    }

    #[test]
    /// Check that the itterative and recursive fibonacci
    /// produce the same value. Both are combinatorial ( F(0) = F(1) = 1 )
    fn test_iterative_and_recursive_equivalence() {
        assert_eq!(fibonacci(0), recursive_fibonacci(0));
        assert_eq!(fibonacci(1), recursive_fibonacci(1));
        assert_eq!(fibonacci(2), recursive_fibonacci(2));
        assert_eq!(fibonacci(3), recursive_fibonacci(3));
        assert_eq!(fibonacci(4), recursive_fibonacci(4));
        assert_eq!(fibonacci(5), recursive_fibonacci(5));
        assert_eq!(fibonacci(10), recursive_fibonacci(10));
        assert_eq!(fibonacci(20), recursive_fibonacci(20));
        assert_eq!(fibonacci(100), recursive_fibonacci(100));
        assert_eq!(fibonacci(184), recursive_fibonacci(184));
    }

    #[test]
    /// Check that classical and combinatorial fibonacci produce the
    /// same value when 'n' differs by 1.
    /// classical fibonacci: ( F(0) = 0, F(1) = 1 )
    /// combinatorial fibonacci: ( F(0) = F(1) = 1 )
    fn test_classical_and_combinatorial_are_off_by_one() {
        assert_eq!(classical_fibonacci(1), fibonacci(0));
        assert_eq!(classical_fibonacci(2), fibonacci(1));
        assert_eq!(classical_fibonacci(3), fibonacci(2));
        assert_eq!(classical_fibonacci(4), fibonacci(3));
        assert_eq!(classical_fibonacci(5), fibonacci(4));
        assert_eq!(classical_fibonacci(6), fibonacci(5));
        assert_eq!(classical_fibonacci(11), fibonacci(10));
        assert_eq!(classical_fibonacci(20), fibonacci(19));
        assert_eq!(classical_fibonacci(21), fibonacci(20));
        assert_eq!(classical_fibonacci(101), fibonacci(100));
        assert_eq!(classical_fibonacci(185), fibonacci(184));
    }

    #[test]
    fn test_memoized_fibonacci() {
        assert_eq!(memoized_fibonacci(0), 0);
        assert_eq!(memoized_fibonacci(1), 1);
        assert_eq!(memoized_fibonacci(2), 1);
        assert_eq!(memoized_fibonacci(3), 2);
        assert_eq!(memoized_fibonacci(4), 3);
        assert_eq!(memoized_fibonacci(5), 5);
        assert_eq!(memoized_fibonacci(10), 55);
        assert_eq!(memoized_fibonacci(20), 6765);
        assert_eq!(memoized_fibonacci(21), 10946);
        assert_eq!(memoized_fibonacci(100), 354224848179261915075);
        assert_eq!(
            memoized_fibonacci(184),
            127127879743834334146972278486287885163
        );
    }

    #[test]
    fn test_matrix_fibonacci() {
        assert_eq!(matrix_fibonacci(0), 0);
        assert_eq!(matrix_fibonacci(1), 1);
        assert_eq!(matrix_fibonacci(2), 1);
        assert_eq!(matrix_fibonacci(3), 2);
        assert_eq!(matrix_fibonacci(4), 3);
        assert_eq!(matrix_fibonacci(5), 5);
        assert_eq!(matrix_fibonacci(10), 55);
        assert_eq!(matrix_fibonacci(20), 6765);
        assert_eq!(matrix_fibonacci(21), 10946);
        assert_eq!(matrix_fibonacci(100), 354224848179261915075);
        assert_eq!(
            matrix_fibonacci(184),
            127127879743834334146972278486287885163
        );
    }
}
