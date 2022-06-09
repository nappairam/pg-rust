// 53. Maximum Subarray
// Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.
//
// A subarray is a contiguous part of an array.
//
//
//
// Example 1:
//
// Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
// Output: 6
// Explanation: [4,-1,2,1] has the largest sum = 6.
// Example 2:
//
// Input: nums = [1]
// Output: 1
// Example 3:
//
// Input: nums = [5,4,-1,7,8]
// Output: 23
//
//
// Constraints:
//
// 1 <= nums.length <= 105
// -104 <= nums[i] <= 104
//
//
// Follow up: If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.

struct Solution;

use std::cmp;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut lmax = 0;
        let mut gmax = i32::MIN;
        for num in nums {
            lmax = cmp::max(num, num + lmax);
            gmax = cmp::max(gmax, lmax);
        }
        gmax
    }

    pub fn max_sub_array1(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((0, i32::MIN), |acc, x| {
                let lmax = cmp::max(x, x + acc.0);
                (lmax, cmp::max(acc.1, lmax))
            })
            .1
    }
}

#[test]
fn test_max_sub_array() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    assert_eq!(
        Solution::max_sub_array(nums.clone()),
        Solution::max_sub_array1(nums)
    );
}
