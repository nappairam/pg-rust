// 2289. Steps to Make Array Non-decreasing
// https://leetcode.com/problems/steps-to-make-array-non-decreasing/
//
// You are given a 0-indexed integer array nums.
// In one step, remove all elements nums[i] where nums[i - 1] > nums[i] for all 0 < i < nums.length.
//
// Return the number of steps performed until nums becomes a non-decreasing array.
//
// Example 1:
//
// Input: nums = [5,3,4,4,7,3,6,11,8,5,11]
// Output: 3
// Explanation: The following are the steps performed:
// - Step 1: [5,3,4,4,7,3,6,11,8,5,11] becomes [5,4,4,7,6,11,11]
// - Step 2: [5,4,4,7,6,11,11] becomes [5,4,7,11,11]
// - Step 3: [5,4,7,11,11] becomes [5,7,11,11]
// [5,7,11,11] is a non-decreasing array. Therefore, we return 3.
//
// Example 2:
// Input: nums = [4,5,7,7,13]
// Output: 0
// Explanation: nums is already a non-decreasing array. Therefore, we return 0.
//
// Constraints:
//
// 1 <= nums.length <= 105
// 1 <= nums[i] <= 109

struct Solution;

impl Solution {
    pub fn total_steps(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut iterations = 0;
        let mut iteration_needed = true;
        // let is_done = |arr: &Vec<i32>| (arr.iter().fold((true, 0), |acc, &x| (acc.0 && acc.1 <= x, x))).0;

        while iteration_needed {
            let mut var: Vec<bool> = vec![];
            nums.iter().fold((&mut var, 0), |mut acc, &x| {
                acc.0.push(acc.1 <= x);
                acc.1 = x;
                acc
            });
            let mut iter = var.iter();
            nums.retain(|_| *iter.next().unwrap());
            iterations += 1;
            iteration_needed = var.contains(&false);
        }

        iterations - 1
    }
}

#[test]
fn test_total_steps() {
    assert_eq!(
        Solution::total_steps(vec![5, 3, 4, 4, 7, 3, 6, 11, 8, 5, 11]),
        3
    )
}
