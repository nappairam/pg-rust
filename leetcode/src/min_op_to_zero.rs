// 1658. Minimum Operations to Reduce X to Zero
// https://leetcode.com/problems/minimum-operations-to-reduce-x-to-zero/
//
// You are given an integer array nums and an integer x. In one operation, you can either remove the leftmost or the rightmost element from the array nums and subtract its value from x. Note that this modifies the array for future operations.
//
// Return the minimum number of operations to reduce x to exactly 0 if it is possible, otherwise, return -1.
//
//
//
// Example 1:
//
// Input: nums = [1,1,4,2,3], x = 5
// Output: 2
// Explanation: The optimal solution is to remove the last two elements to reduce x to zero.
// Example 2:
//
// Input: nums = [5,6,7,8,9], x = 4
// Output: -1
// Example 3:
//
// Input: nums = [3,2,20,1,1,3], x = 10
// Output: 5
// Explanation: The optimal solution is to remove the last three elements and the first two elements (5 operations in total) to reduce x to zero.
//
//
// Constraints:
//
// 1 <= nums.length <= 10^5
// 1 <= nums[i] <= 10^4
// 1 <= x <= 10^9

struct Solution;

const DEBUG: i32 = 0;

use std::cmp;
use std::cmp::Ordering;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = (0_i32, nums[0]);
        let mut right = (nums.len() as i32, 0);

        let mut result = i32::MAX;
        // println!("========> Start <=========");

        while left.0 < right.0 - 1 && left.1 < target {
            let next_index = left.0 + 1;
            left = (next_index, left.1 + nums[next_index as usize]);
        }
        // Only left half result
        if left.1 == target {
            result = left.0 + 1;
        }
        // Reduce one element so that the sum is lesser than target
        left = (left.0 - 1, left.1 - nums[left.0 as usize]);
        // println!("After left loop, left:{:?} right:{:?} result:{}", left, right, result);

        // Decrement one index
        while left.0 < right.0 - 1 && right.0 - 1 >= 0 {
            // Move the right pointer backwards
            right = (right.0 - 1, right.1 + nums[right.0 as usize - 1]);
            // println!("Inside loop, left:{:?} right:{:?} result:{}", left, right, result);

            match target.cmp(&(left.1 + right.1)) {
                Ordering::Equal => {
                    // println!("Result equal");
                    result = cmp::min(left.0 + (nums.len() as i32 - right.0) + 1, result);
                }
                Ordering::Less => {
                    // println!("Result lesser, {} < {}", target, left.1 + right.1);
                    if left.0 >= 0 {
                        let next_index = left.0 - 1;
                        left = (next_index, left.1 - nums[left.0 as usize]);
                        // Move the right pointer forwards to check again
                        right = (right.0 + 1, right.1 - nums[right.0 as usize]);
                        // println!("Resetting left {:?} right {:?}", left, right);
                    }
                }
                Ordering::Greater => {}
            }
        }
        if result != i32::MAX { result } else { -1 }
    }
}

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(vec![1, 1], 3), -1);
    assert_eq!(Solution::min_operations(vec![5, 2, 3, 1, 1], 5), 1);
    assert_eq!(Solution::min_operations(vec![3, 2, 20, 1, 1, 3], 10), 5);
    assert_eq!(Solution::min_operations(vec![5, 6, 7, 8, 9], 4), -1);
    assert_eq!(Solution::min_operations(vec![1, 1, 4, 2, 3], 5), 2);
}