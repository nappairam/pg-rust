// 322. Coin Change
// https://leetcode.com/problems/coin-change/
//
// You are given an integer array coins representing coins of different
// denominations and an integer amount representing a total amount of money.
//
// Return the fewest number of coins that you need to make up that amount.
// If that amount of money cannot be made up by any combination of the coins, return -1.
// You may assume that you have an infinite number of each kind of coin.
//
// Example 1:
// Input: coins = [1,2,5], amount = 11
// Output: 3
// Explanation: 11 = 5 + 5 + 1
//
// Example 2:
// Input: coins = [2], amount = 3
// Output: -1
//
// Example 3:
// Input: coins = [1], amount = 0
// Output: 0
//
// Constraints:
// 1 <= coins.length <= 12
// 1 <= coins[i] <= 231 - 1
// 0 <= amount <= 104

struct Solution;

use std::cmp;
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![i32::MAX; 1 + amount as usize];
        dp[0] = 0;
        let mut coins = coins.clone();
        coins.sort();

        for x in 1..amount + 1 {
            let b = coins
                .iter()
                .filter(|&&c| c <= x)
                .map(|c| x - c)
                .fold(i32::MAX, |_a, c| {
                    // println!("c is {} x is {} {:?}", c, x, dp);
                    if dp[c as usize] != i32::MAX {
                        cmp::min(1 + dp[c as usize], dp[x as usize])
                    } else {
                        i32::MAX
                    }
                });
            dp[x as usize] = b;
            // println!("{:?}", dp);
        }
        let result = dp[amount as usize];
        if result == i32::MAX {
            -1
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_change() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
        assert_eq!(Solution::coin_change(vec![2, 5, 10, 1], 27), 4);
        assert_eq!(Solution::coin_change(vec![2, 5, 10, 1], 27), 4);
    }
}
