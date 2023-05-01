//! All combinations of size k
//!
//! In this problem, we want to determine all possible combinations of k
//! numbers out of 1 ... n. We use backtracking to solve this problem.
//!
//! # Time Complexity
//! - O(C(n,k)) which is O(n choose k) = O((n!/(k! * (n - k)!)))
//!
//! # Examples
//! ```
//! use algorithms::backtracking::generate_all_combinations;
//! let combos = generate_all_combinations(4, 2);
//! assert_eq!(combos, vec![
//!     vec![1, 2],
//!     vec![1, 3],
//!     vec![1, 4],
//!     vec![2, 3],
//!     vec![2, 4],
//!     vec![3, 4]
//! ]);
//! ```

/// Generate all combinations of size k out of 1 ... n
///
/// # Examples
/// ```
/// use algorithms::backtracking::generate_all_combinations;
/// let combos = generate_all_combinations(4, 2);
/// assert_eq!(combos, vec![
///     vec![1, 2],
///     vec![1, 3],
///     vec![1, 4],
///     vec![2, 3],
///     vec![2, 4],
///     vec![3, 4]
/// ]);
/// ```
pub fn generate_all_combinations(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    create_all_state(1, n, k, &mut vec![], &mut result);
    result
}

fn create_all_state(
    increment: i32,
    total_number: i32,
    level: i32,
    current_list: &mut Vec<i32>,
    total_list: &mut Vec<Vec<i32>>,
) {
    if level == 0 {
        total_list.push(current_list.clone());
        return;
    }

    for i in increment..(total_number - level + 2) {
        current_list.push(i);
        create_all_state(i + 1, total_number, level - 1, current_list, total_list);
        current_list.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        let expected = vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
        ];
        let result = generate_all_combinations(4, 2);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_empty() {
        let expected: Vec<Vec<i32>> = vec![vec![]];
        let result = generate_all_combinations(0, 0);

        assert_eq!(expected, result);
    }
}
