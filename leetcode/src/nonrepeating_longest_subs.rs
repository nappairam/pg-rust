// 3. Longest Substring Without Repeating Characters
// https://leetcode.com/problems/longest-substring-without-repeating-characters/
//
// Given a string s, find the length of the longest substring without repeating characters.
//
//
//
// Example 1:
//
// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3.
// Example 2:
//
// Input: s = "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.
// Example 3:
//
// Input: s = "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3.
// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
//
//
// Constraints:
//
// 0 <= s.length <= 5 * 104
// s consists of English letters, digits, symbols and spaces.

struct Solution;

use std::cmp;
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut start = 0;
        let mut longest = 0;
        let mut dp: HashMap<char, usize> = HashMap::new();

        for (index, ch) in s.chars().enumerate() {
            if dp.contains_key(&ch) {
                if let Some(c) = dp.get_mut(&ch) {
                    start = cmp::max(start, *c + 1);
                    *c = index;
                }
            } else {
                dp.insert(ch, index);
            }
            longest = cmp::max(longest, index + 1 - start);
            // println!("Dp is {:?} start:{} index:{} ch:{} long: {}", dp, start, index, ch, longest);
        }
        longest as i32
    }
}

#[test]
fn test_longest_substring() {
    let test_input = vec![
        ("abcabcbb", 3),
        ("pwwkew", 3),
        ("", 0),
        ("abba", 2)
    ];

    for input in test_input {
        assert_eq!(Solution::length_of_longest_substring(input.0.to_owned()), input.1);
    }
}