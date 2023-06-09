//! Coin change via dynamic programming
use itertools::Itertools;
use num_traits::PrimInt;

/// [`coin_change`] returns the fewest number of coins that need to make up that
/// amount. If that amount of money cannot be made up by any combination of the
/// coins, return `None`.
///
/// ## Arguments:
///   * `coins` - coins of different denominations
///   * `amount` - a total amount of money be made up.
///
/// ## Complexity
///   - time complexity: O(amount * coins.length),
///   - space complexity: O(amount),
pub fn coin_change<T: PrimInt>(coins: &[T], amount: usize) -> Option<T> {
    let mut dp = (0..=amount)
        .map(|n| if n == 0 { Some(T::zero()) } else { None })
        .collect_vec();

    for (i, coin) in (0..=amount)
        .cartesian_product(coins.iter().filter_map(|c| c.to_usize()))
        .filter(|(i, c)| i >= c)
    {
        if let Some(prev_coins) = dp[i - coin] {
            dp[i] = dp[i].map_or(Some(prev_coins + T::one()), |curr_coins| {
                Some(curr_coins.min(prev_coins + T::one()))
            })
        }
    }
    dp[amount]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // 11 = 5 * 2 + 1 * 1
        let coins = vec![1, 2, 5];
        assert_eq!(Some(3), coin_change(&coins, 11));

        // 119 = 11 * 10 + 7 * 1 + 2 * 1
        let coins = vec![2, 3, 5, 7, 11];
        assert_eq!(Some(12), coin_change(&coins, 119));
    }

    #[test]
    fn coins_empty() {
        let coins: Vec<usize> = vec![];
        assert_eq!(None, coin_change(&coins, 1));
    }

    #[test]
    fn amount_zero() {
        let coins = vec![1, 2, 3];
        assert_eq!(Some(0), coin_change(&coins, 0));
    }

    #[test]
    fn fail_change() {
        // 3 can't be change by 2.
        let coins: Vec<isize> = vec![2];
        assert_eq!(None, coin_change(&coins, 3));

        let coins = vec![10, 20, 50, 100];
        assert_eq!(None, coin_change(&coins, 5));
    }
}
