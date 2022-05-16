#![allow(dead_code)]
// 41. First Missing Positive
//
// Given an unsorted integer array nums, return the smallest missing positive integer.
//
// You must implement an algorithm that runs in O(n) time and uses constant extra space.
//
//
//
// Example 1:
//
// Input: nums = [1,2,0]
// Output: 3
// Example 2:
//
// Input: nums = [3,4,-1,1]
// Output: 2
// Example 3:
//
// Input: nums = [7,8,9,11,12]
// Output: 1
//
//
// Constraints:
//
// 1 <= nums.length <= 5 * 105
// -2^31 <= nums[i] <= (2^31) - 1
use std::cmp;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let nums_len = nums.len();
        let mut dp: Vec<bool> = vec![false; nums_len + 1];
        // for num in &nums {
        //     if *num < 1 || *num > nums_len as i32 {
        //         continue;
        //     }
        //     dp[*num as usize] = true;
        // }
        nums.iter().filter(|&&x| x > 0 && x <= nums_len as i32).for_each(|&x| dp[x as usize] = true);
        (dp.iter().skip(1).position(|&value| !value).unwrap_or(nums_len) + 1) as i32
    }

    pub fn first_missing_positive_naive(nums: Vec<i32>) -> i32 {
        let mut dp: HashMap<i32, bool> = HashMap::new();
        let mut largest = 0;
        for num in &nums {
            if *num < 1 {
                continue;
            }
            dp.insert(*num, true);
            largest = cmp::max(largest, *num);
        }
        for i in 1..=largest {
            if !dp.contains_key(&(i as i32)) {
                return i as i32;
            }
        }
        largest + 1
    }
}

#[test]
fn test_first_missing_positive() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
    assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
    assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    assert_eq!(Solution::first_missing_positive(vec![2147483647]), 1);
    assert_eq!(Solution::first_missing_positive(vec![1]), 2);
}
