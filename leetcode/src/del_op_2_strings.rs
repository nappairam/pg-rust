// 583. Delete Operation for Two Strings
// Given two strings word1 and word2, return the minimum number of steps required
// to make word1 and word2 the same.
//
// In one step, you can delete exactly one character in either string.
//
// Example 1:
// Input: word1 = "sea", word2 = "eat"
// Output: 2
// Explanation: You need one step to make "sea" to "ea" and another step to make "eat" to "ea".
//
// Example 2:
// Input: word1 = "leetcode", word2 = "etco"
// Output: 4
//
// Constraints:
// * 1 <= word1.length, word2.length <= 500
// * word1 and word2 consist of only lowercase English letters.

struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (word1, word2) = if word1.len() < word2.len() { (word1, word2) } else { (word2, word1) };
        let mut dp = vec![vec![0; word2.len()]; word1.len()];

        for (i, ch1) in word1.chars().enumerate() {
            for (j, ch2) in word2.chars().enumerate() {
                if ch1 == ch2 {
                    dp[i][j] = 1 + if i > 0 && j > 0 { dp[i - 1][j - 1] } else { 0 };
                } else {
                    if i > 0 && j > 0 {
                        dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
                    } else if j > 0 {
                        dp[i][j] = dp[i][j - 1];
                    } else if i > 0 {
                        dp[i][j] = dp[i - 1][j]
                    }
                }
            }
        }
        println!("{:?}", dp);
        (word1.len() + word2.len() - (2 * dp[word1.len() - 1][word2.len() - 1])) as i32
    }
}

#[test]
fn test_min_distance() {
    assert_eq!(Solution::min_distance("sea".to_owned(), "eat".to_owned()), 2);
    // assert_eq!(Solution::min_distance("a".to_owned(), "ab".to_owned()), 1);
    // assert_eq!(Solution::min_distance("leetcode".to_owned(), "etco".to_owned()), 4);
    // assert_eq!(Solution::min_distance("etco".to_owned(), "leetcode".to_owned()), 4);
}