// 915. Partition Array into Disjoint Intervals
// https://leetcode.com/problems/partition-array-into-disjoint-intervals/
//
// Given an integer array nums, partition it into two (contiguous) subarrays left and right so that:
//
// Every element in left is less than or equal to every element in right.
// left and right are non-empty.
// left has the smallest possible size.
// Return the length of left after such a partitioning.
//
// Test cases are generated such that partitioning exists.
//
//
//
// Example 1:
//
// Input: nums = [5,0,3,8,6]
// Output: 3
// Explanation: left = [5,0,3], right = [8,6]
// Example 2:
//
// Input: nums = [1,1,1,0,6,12]
// Output: 4
// Explanation: left = [1,1,1,0], right = [6,12]
//
//
// Constraints:
//
// 2 <= nums.length <= 105
// 0 <= nums[i] <= 106
// There is at least one valid answer for the given input.

struct Solution;

impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let mut lbiggest = nums[0];
        let mut biggest = lbiggest;
        let mut result = 0;

        nums.into_iter().enumerate().for_each(|(i, num)|{
            if num > biggest {
                biggest = num;
            }
            if num < lbiggest {
                result = i as i32;
                lbiggest = biggest;
            }
        });
        result + 1
    }
}

#[test]
fn test_partition_disjoint() {
    assert_eq!(Solution::partition_disjoint(vec![1,1,1,0,6,12]), 4);
    assert_eq!(Solution::partition_disjoint(vec![5,0,3,8,6]), 3);
    assert_eq!(Solution::partition_disjoint(vec![1,1]), 1);
}


