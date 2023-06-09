use num_traits::PrimInt;

/// Finds the subarray (containing at least one number) which has the largest
/// sum and return its sum.
///
/// A subarray is a contiguous part of an array.
///
/// ## Complexity
/// - time complexity: O(n)
/// - space complexity: O(n)
pub fn maximum_subarray<T: PrimInt>(arr: &[T]) -> T {
    let mut dp = vec![T::zero(); arr.len()];
    dp[0] = arr[0];

    for i in 1..arr.len() {
        dp[i] = if dp[i - 1] > T::zero() {
            dp[i - 1] + arr[i]
        } else {
            arr[i]
        };
    }
    *dp.iter().max().unwrap_or(&dp[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_negative() {
        // the maximum value: 1 + 0 + 5 + 8 = 14
        let array = vec![1, 0, 5, 8];
        assert_eq!(maximum_subarray(&array), 14);
    }

    #[test]
    fn negative() {
        // the maximum value: -1
        let array = vec![-3, -1, -8, -2];
        assert_eq!(maximum_subarray(&array), -1);
    }

    #[test]
    fn normal() {
        // the maximum value: 3 + (-2) + 5 = 6
        let array = vec![-4, 3, -2, 5, -8];
        assert_eq!(maximum_subarray(&array), 6);
    }

    #[test]
    fn single_element() {
        let array = vec![6];
        assert_eq!(maximum_subarray(&array), 6);
        let array = vec![-6];
        assert_eq!(maximum_subarray(&array), -6);
    }
}
