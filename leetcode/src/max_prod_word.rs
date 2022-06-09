// 318. Maximum Product of Word Lengths
// https://leetcode.com/problems/maximum-product-of-word-lengths/
//
// Given a string array words, return the maximum value of length(word[i]) * length(word[j]) where the two words do not share common letters. If no such two words exist, return 0.
//
//
//
// Example 1:
//
// Input: words = ["abcw","baz","foo","bar","xtfn","abcdef"]
// Output: 16
// Explanation: The two words can be "abcw", "xtfn".
// Example 2:
//
// Input: words = ["a","ab","abc","d","cd","bcd","abcd"]
// Output: 4
// Explanation: The two words can be "ab", "cd".
// Example 3:
//
// Input: words = ["a","aa","aaa","aaaa"]
// Output: 0
// Explanation: No such pair of words.
//
//
// Constraints:
//
// 2 <= words.length <= 1000
// 1 <= words[i].length <= 1000
// words[i] consists only of lowercase English letters.

struct Solution;

use std::cmp;

impl Solution {
    pub fn max_product(words: Vec<&str>) -> i32 {
        let total_words = words.len();
        let mut dp: Vec<u32> = Vec::with_capacity(total_words);
        let mut max = 0;

        for word in words.iter() {
            let mut bitmap: u32 = 0;
            for letter in word.chars() {
                bitmap |= 1 << (letter as u32 - 'a' as u32);
            }
            dp.push(bitmap);
        }
        for (index, _) in dp.iter().enumerate() {
            for i in index + 1..total_words {
                // println!("Comparing {} {} with bitmask {:b} {:b}", words[index], words[i], dp[index], dp[i]);
                if dp[index] & dp[i] == 0 {
                    max = cmp::max(max, words[index].len() * words[i].len())
                }
            }
        }
        max as i32
    }
}

#[test]
fn test_max_product() {
    assert_eq!(
        Solution::max_product(vec!["abcw", "baz", "foo", "bar", "xtfn", "abcdef"]),
        16
    );
    assert_eq!(
        Solution::max_product(vec!["a", "ab", "abc", "d", "cd", "bcd", "abcd"]),
        4
    );
    assert_eq!(Solution::max_product(vec!["a", "aa", "aaa", "aaaa"]), 0);
}

#[test]
fn test_cmp() {
    assert_eq!(cmp::max(0, -1), -1);
}
