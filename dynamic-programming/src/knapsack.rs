//! Solves the knapsack problem

use std::cmp;

// use itertools::Itertools;
use num_traits::PrimInt;

/// [`knapsack_table`] returns the knapsack table (`n`, `m`) with maximum
/// values, where `n` is number of items
///
/// ## Arguments:
///   * `w` - knapsack capacity
///   * `weights` - set of weights for each item
///   * `values` - set of values for each item
fn knapsack_table<T: PrimInt>(w: &usize, weights: &[T], values: &[T]) -> Vec<Vec<T>> {
    let n = weights.len();
    let mut m = vec![vec![T::zero(); w + 1]; n + 1];

    for i in 0..=n {
        for j in 0..=*w {
            if i == 0 || j == 0 {
                m[i][j] = T::zero();
            } else if weights[i - 1].to_usize().unwrap() <= j {
                m[i][j] = cmp::max(
                    values[i - 1] + m[i - 1][j - weights[i - 1].to_usize().unwrap()],
                    m[i - 1][j],
                );
            } else {
                m[i][j] = m[i - 1][j];
            }
        }
    }
    m
}

/// [`knapsack_items`] returns the indices of the items of the optimal knapsack
/// (from `1` to `n`)
///
/// ## Arguments:
///   * `weights` - set of weights for each item
///   * `m` - knapsack table with maximum values
///   * `i` - include items 1 through `i` in knapsack (for the initial value,
///     use `n`)
///   * `j` - maximum weight of the knapsack
fn knapsack_items<T: PrimInt>(weights: &[T], m: &[Vec<T>], i: usize, j: usize) -> Vec<T> {
    if i == 0 {
        return vec![];
    }
    if m[i][j] > m[i - 1][j] {
        let mut knap = knapsack_items(weights, m, i - 1, j - weights[i - 1].to_usize().unwrap());
        knap.push(T::from(i).unwrap());
        knap
    } else {
        knapsack_items(weights, m, i - 1, j)
    }
}

/// [`knapsack`] returns the tuple where first value is "optimal profit", second
/// value is "knapsack optimal weight" and the last value is "indices of items",
/// that we got (from 1 to `n`)
///
/// ## Arguments:
///   * `w` - knapsack capacity
///   * `weights` - set of weights for each item
///   * `values` - set of values for each item
///
/// ## Complexity
///   - time complexity: O(nw),
///   - space complexity: O(nw),
///
/// where `n` and `w` are "number of items" and "knapsack capacity"
pub fn knapsack<T: PrimInt>(w: usize, weights: Vec<T>, values: Vec<T>) -> (T, T, Vec<T>) {
    assert_eq!(
        weights.len(),
        values.len(),
        "Number of items in the list of weights doesn't match the number of items in the list of values!"
    );
    let n = weights.len();
    let m = knapsack_table(&w, &weights, &values);
    let items = knapsack_items(&weights, &m, n, w);
    let total_weight = items.iter().fold(T::zero(), |acc, i| {
        acc + weights[(*i - T::one()).to_usize().unwrap()]
    });
    (m[n][w], total_weight, items)
}

#[cfg(test)]
mod tests {
    // Took test datasets from https://people.sc.fsu.edu/~jburkardt/datasets/bin_packing/bin_packing.html
    use super::*;

    #[test]
    fn test_p02() {
        assert_eq!(
            (51, 26, vec![2, 3, 4]),
            knapsack(26, vec![12, 7, 11, 8, 9], vec![24, 13, 23, 15, 16])
        );
    }

    #[test]
    fn test_p04() {
        assert_eq!(
            (150, 190, vec![1, 2, 5]),
            knapsack(190, vec![56, 59, 80, 64, 75, 17], vec![
                50, 50, 64, 46, 50, 5
            ])
        );
    }

    #[test]
    fn test_p01() {
        assert_eq!(
            (309, 165, vec![1, 2, 3, 4, 6]),
            knapsack(165, vec![23, 31, 29, 44, 53, 38, 63, 85, 89, 82], vec![
                92, 57, 49, 68, 60, 43, 67, 84, 87, 72
            ])
        );
    }

    #[test]
    fn test_p06() {
        assert_eq!(
            (1735, 169, vec![2, 4, 7]),
            knapsack(170, vec![41, 50, 49, 59, 55, 57, 60], vec![
                442, 525, 511, 593, 546, 564, 617
            ])
        );
    }

    #[test]
    fn test_p07() {
        assert_eq!(
            (1458, 749, vec![1, 3, 5, 7, 8, 9, 14, 15]),
            knapsack(
                750,
                vec![
                    70, 73, 77, 80, 82, 87, 90, 94, 98, 106, 110, 113, 115, 118, 120
                ],
                vec![
                    135, 139, 149, 150, 156, 163, 173, 184, 192, 201, 210, 214, 221, 229, 240
                ]
            )
        );
    }
}
