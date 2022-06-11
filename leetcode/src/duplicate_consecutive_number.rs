// 287. Find the Duplicate Number
// https://leetcode.com/problems/find-the-duplicate-number
//
// Given an array of integers nums containing n + 1 integers where each integer is in the range [1, n] inclusive.
//
// There is only one repeated number in nums, return this repeated number.
//
// You must solve the problem without modifying the array nums and uses only constant extra space.
//
//
//
// Example 1:
//
// Input: nums = [1,3,4,2,2]
// Output: 2
// Example 2:
//
// Input: nums = [3,1,3,4,2]
// Output: 3
//
//
// Constraints:
//
// 1 <= n <= 105
// nums.length == n + 1
// 1 <= nums[i] <= n
// All the integers in nums appear only once except for precisely one integer which appears two or more times.
//
//
// Follow up:
//
// How can we prove that at least one duplicate number must exist in nums?
// Can you solve the problem in linear runtime complexity?

struct Solution;

use std::cmp;
use std::collections::HashSet;

impl Solution {
    pub fn find_one_duplicate(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut sum = 0;
        let count = nums.len() as i32 - 1;

        for num in nums {
            sum += num;
            min = cmp::min(min, num);
        }
        let sequence_sum = ((min - 1) * count) + (count * (count + 1) / 2);
        sum - sequence_sum
    }

    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut dp: HashSet<i32> = HashSet::new();
        *nums.iter().find(|&&x| false == dp.insert(x)).unwrap()
    }
}

#[test]
fn test_find_dup() {
    assert_eq!(Solution::find_one_duplicate(vec![1, 3, 4, 2, 2]), 2);
    assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
}