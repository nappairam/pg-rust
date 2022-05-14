// 32. Longest Valid Parentheses
// Given a string containing just the characters '(' and ')',
// find the length of the longest valid (well-formed) parentheses substring.
//
// Example 1:
//
// Input: s = "(()"
// Output: 2
// Explanation: The longest valid parentheses substring is "()".
// Example 2:
//
// Input: s = ")()())"
// Output: 4
// Explanation: The longest valid parentheses substring is "()()".
// Example 3:
//
// Input: s = ""
// Output: 0
//
//
// Constraints:
//
// 0 <= s.length <= 3 * 104
// s[i] is '(', or ')'


use std::cmp;

pub struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut sta = vec![];
        let mut longest_count = 0;
        let mut prev_count = 0;
        let mut count = 0;
        for c in s.chars() {
            match c {
                '(' => {
                    if sta.is_empty() {
                        prev_count = count;
                        longest_count = cmp::max(longest_count, count);
                        count = 0;
                    }
                    sta.push(c)
                },
                ')' => {
                    if sta.is_empty() {
                        longest_count = cmp::max(longest_count, count + prev_count);
                        count = 0;
                        continue;
                    }
                    sta.pop();
                    count += 2;
                    if sta.is_empty() {
                        count += prev_count;
                        prev_count = 0;
                    }
                },
                _ => continue
            }
        }
        cmp::max(longest_count, count)
    }
}

#[test]
#[ignore]
fn test_longest_valid_parentheses() {
    assert_eq!(Solution::longest_valid_parentheses("()".to_owned()), 2);
    assert_eq!(Solution::longest_valid_parentheses("(()".to_owned()), 2);
    assert_eq!(Solution::longest_valid_parentheses(")()())".to_owned()), 4);
    assert_eq!(Solution::longest_valid_parentheses("".to_owned()), 0);
    assert_eq!(Solution::longest_valid_parentheses("((())".to_owned()), 4);
    assert_eq!(Solution::longest_valid_parentheses(")())())".to_owned()), 2);
    assert_eq!(Solution::longest_valid_parentheses("()(()".to_owned()), 2);
    assert_eq!(Solution::longest_valid_parentheses("(())(".to_owned()), 4);
    assert_eq!(Solution::longest_valid_parentheses("(()(((()".to_owned()), 2);
}