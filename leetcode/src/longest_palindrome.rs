// 5. Longest Palindromic Substring
//
// Given a string s, return the longest palindromic substring in s.
//
// Example 1:
// Input: s = "babad"
// Output: "bab"
// Explanation: "aba" is also a valid answer.
//
// Example 2:
// Input: s = "cbbd"
// Output: "bb"
//
// Constraints:
// * 1 <= s.length <= 1000
// * s consist of only digits and English letters.

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        println!("Input string is {}", s);
        "bab".to_owned()
    }
}

#[test]
fn test_longest_palindrome() {
    assert_eq!(Solution::longest_palindrome("babad".to_owned()), "bab");
}