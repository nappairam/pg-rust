// 1461. Check If a String Contains All Binary Codes of Size K
// https://leetcode.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k/
//
// Given a binary string s and an integer k, return true if every binary code of length k
// is a substring of s. Otherwise, return false.
//
// Example 1:
// Input: s = "00110110", k = 2
// Output: true
// Explanation: The binary codes of length 2 are "00", "01", "10" and "11". They can be all found as substrings at indices 0, 1, 3 and 2 respectively.
//
// Example 2:
// Input: s = "0110", k = 1
// Output: true
// Explanation: The binary codes of length 1 are "0" and "1", it is clear that both exist as a substring.
//
// Example 3:
// Input: s = "0110", k = 2
// Output: false
// Explanation: The binary code "00" is of length 2 and does not exist in the array.
//
//
// Constraints:
//
// 1 <= s.length <= 5 * 105
// s[i] is either '0' or '1'.
// 1 <= k <= 20

struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let mut codes: HashSet<&str> = HashSet::new();
        let k: usize = k as usize;
        let slen = s.len();
        let total_len = 2_usize.pow(k as u32);

        if slen < k {
            return false;
        }

        for i in 0..=slen - k {
            codes.insert(&s[i..i + k]);
            if codes.len() == total_len {
                return true;
            }
        }
        // println!("K is {} Codes are {:?} {} == {}", k, codes, codes.len(), 2_i32.pow(k as u32));
        false
    }
}

#[test]
fn test_has_all_code() {
    assert_eq!(true, Solution::has_all_codes("00110110".to_owned(), 2));
}
