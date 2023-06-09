use std::cmp;

use num_traits::PrimInt;

/// Maximal Square
///
/// Given an `m` * `n` binary matrix filled with 0's and 1's, find the largest
/// square containing only 1's and return its area. <https://leetcode.com/problems/maximal-square/>
///
/// # Arguments:
///   * `matrix` - an array of integer array
///
/// # Complexity
///   - time complexity: O(n^2),
///   - space complexity: O(n),
pub fn maximal_square<T: PrimInt>(matrix: &mut [Vec<T>]) -> T {
    if matrix.is_empty() {
        return T::zero();
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut result = T::zero();

    for row in 0..rows {
        for col in 0..cols {
            if matrix[row][col].is_one() {
                result = if row == 0 || col == 0 {
                    cmp::max(result, T::one())
                } else {
                    let count = cmp::min(
                        cmp::min(matrix[row - 1][col - 1], matrix[row - 1][col]),
                        matrix[row][col - 1],
                    ) + T::one();
                    matrix[row][col] = count;
                    cmp::max(result, count)
                };
            }
        }
    }
    result * result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(maximal_square::<i32>(&mut []), 0);

        let mut matrix = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(maximal_square(&mut matrix), 1);

        let mut matrix = vec![
            vec![1, 0, 1, 0, 0],
            vec![1, 0, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 0, 1, 0],
        ];
        assert_eq!(maximal_square(&mut matrix), 4);

        let mut matrix = vec![vec![0]];
        assert_eq!(maximal_square(&mut matrix), 0);
    }
}
