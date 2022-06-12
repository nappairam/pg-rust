// 1695. Maximum Erasure Value
// https://leetcode.com/problems/maximum-erasure-value/
// Similar to 3. Longest Substring Without Repeating Characters
//
// You are given an array of positive integers nums and want to erase
// a subarray containing unique elements. The score you get by erasing the
// subarray is equal to the sum of its elements.
//
// Return the maximum score you can get by erasing exactly one subarray.
//
// An array b is called to be a subarray of a if it forms a contiguous subsequence of a,
// that is, if it is equal to a[l],a[l+1],...,a[r] for some (l,r).
//
// Example 1:
// Input: nums = [4,2,4,5,6]
// Output: 17
// Explanation: The optimal subarray here is [2,4,5,6].
//
// Example 2:
// Input: nums = [5,2,1,2,5,2,1,2,5]
// Output: 8
// Explanation: The optimal subarray here is [5,2,1] or [1,2,5].
//
// Constraints:
// 1 <= nums.length <= 10^5
// 1 <= nums[i] <= 10^4

use std::cmp;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut result = i32::MIN;
        let mut pref_sum = 0;
        let mut dp: HashMap<i32, (usize, i32)> = HashMap::new();

        for (index, num) in nums.iter().enumerate() {
            pref_sum += num;
            if dp.contains_key(&num) {
                if let Some(value) = dp.get_mut(&num) {
                    start = cmp::max(start, value.0 + 1);
                    *value = (index, pref_sum);
                }
            } else {
                dp.insert(*num, (index, pref_sum));
            }
            let get_dp_value = |index| dp.get(&nums[index]).unwrap_or(&(0, 0)).1;
            result = cmp::max(result, get_dp_value(index) - get_dp_value(start) + nums[start]);
            // println!("Dp is {:?} start:{} {} index:{} num:{} long: {}", dp, start, start_value, index, num, longest);
        }
        result
    }
}

#[test]
fn test_max_uniq_subarray() {
    assert_eq!(Solution::maximum_unique_subarray(vec![4, 2, 4, 5, 6]), 17);
}