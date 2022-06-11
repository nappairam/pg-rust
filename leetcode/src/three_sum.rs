// 15. 3Sum
//
// https://leetcode.com/problems/3sum/
//
// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]]
// such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
//
// Notice that the solution set must not contain duplicate triplets.
//
// Example 1:
// Input: nums = [-1,0,1,2,-1,-4]
// Output: [[-1,-1,2],[-1,0,1]]
//
// Example 2:
// Input: nums = []
// Output: []
//
// Example 3:
// Input: nums = [0]
// Output: []
//
// Constraints:
// 0 <= nums.length <= 3000
// -105 <= nums[i] <= 105

use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut nums = nums;
        nums.sort();

        for idx1 in 0..nums.len() {
            if idx1 > 0 && nums[idx1] == nums[idx1 - 1] {
                continue;
            }

            let mut idx2 = idx1 + 1;
            let mut idx3 = nums.len() - 1;

            while idx2 < idx3 {
                let sum = nums[idx1] + nums[idx2] + nums[idx3];
                match sum.cmp(&0) {
                    Ordering::Equal => {
                        result.push(vec![nums[idx1], nums[idx2], nums[idx3]]);
                        idx3 -= 1;
                        while idx3 > idx2 && nums[idx3] == nums[idx3 + 1] {
                            idx3 -= 1;
                        }
                    }
                    Ordering::Less => idx2 += 1,
                    Ordering::Greater => idx3 -= 1,
                }
            }
        }
        result
    }
}

#[test]
fn test_three_sum() {
    assert_eq!(Solution::three_sum(vec![1, 1, -2, 4]), vec![vec![-2, 1, 1]]);
}
