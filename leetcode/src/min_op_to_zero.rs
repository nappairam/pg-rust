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

use std::cmp;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

struct Solution;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
enum Index {
    InvalidUnderflow,
    Valid(i32),
    InvalidOverflow,
}

#[derive(Debug)]
enum StepValue {
    Inc,
    Dec,
}

impl StepValue {
    fn value(&self) -> i32 {
        match *self {
            StepValue::Inc => 1,
            StepValue::Dec => -1,
        }
    }
}

#[derive(Debug)]
struct Sequence<'a> {
    values: &'a Vec<i32>,
    len: i32,
    index: Index,
    sum: i32,
    step: StepValue,
}

impl<'a> Display for Sequence<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sequence index:{:?} sum:{:?}", self.index, self.sum)
    }
}

impl<'a> Sequence<'a> {
    fn left(values: &'a Vec<i32>) -> Self {
        Self { values, len: values.len() as i32, index: Index::InvalidUnderflow, sum: 0, step: StepValue::Inc }
    }

    fn right(values: &'a Vec<i32>) -> Self {
        Self { values, len: values.len() as i32, index: Index::InvalidOverflow, sum: 0, step: StepValue::Dec }
    }

    fn is_safe(&self, index: i32) -> bool {
        index < self.len as i32 && index >= 0
    }

    fn next_index(&self) -> Index {
        match (&self.step, &self.index) {
            (StepValue::Inc, Index::InvalidUnderflow) => Index::Valid(0),
            (StepValue::Inc, Index::InvalidOverflow) => Index::InvalidOverflow,
            (StepValue::Inc, Index::Valid(index)) => {
                let next_index = index + self.step.value();
                if self.is_safe(next_index) { Index::Valid(next_index) } else { Index::InvalidOverflow }
            }
            (StepValue::Dec, Index::InvalidUnderflow) => Index::InvalidUnderflow,
            (StepValue::Dec, Index::InvalidOverflow) => Index::Valid(self.len - 1),
            (StepValue::Dec, Index::Valid(index)) => {
                let next_index = index + self.step.value();
                if self.is_safe(next_index) { Index::Valid(next_index) } else { Index::InvalidUnderflow }
            }
        }
    }

    fn prev_index(&self) -> Index {
        match (&self.step, &self.index) {
            (StepValue::Inc, Index::InvalidUnderflow) => Index::InvalidUnderflow,
            (StepValue::Inc, Index::InvalidOverflow) => Index::Valid(self.len - 1),
            (StepValue::Inc, Index::Valid(index)) => {
                let next_index = index - self.step.value();
                if self.is_safe(next_index) { Index::Valid(next_index) } else { Index::InvalidUnderflow }
            }
            (StepValue::Dec, Index::InvalidUnderflow) => Index::Valid(0),
            (StepValue::Dec, Index::InvalidOverflow) => Index::InvalidOverflow,
            (StepValue::Dec, Index::Valid(index)) => {
                let next_index = index - self.step.value();
                if self.is_safe(next_index) { Index::Valid(next_index) } else { Index::InvalidOverflow }
            }
        }
    }

    fn move_next(&mut self) {
        let next_index = self.next_index();
        if let Index::Valid(value) = next_index {
            self.sum += self.values[value as usize];
        }
        self.index = next_index;
    }

    fn move_prev(&mut self) {
        let prev_index = self.prev_index();
        if let Index::Valid(value) = self.index {
            self.sum -= self.values[value as usize];
        }
        self.index = prev_index;
    }
}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, target: i32) -> i32 {
        let mut result = (Index::InvalidUnderflow, Index::InvalidOverflow);
        let mut left = Sequence::left(&nums);
        let mut right = Sequence::right(&nums);
        let mut value = i32::MAX;

        let result_value = |result: &(Index, Index)| match result {
            (Index::Valid(lower), Index::Valid(upper)) => lower + 1 + (nums.len() as i32) - upper,
            (Index::Valid(lower), _) => lower + 1,
            (_, Index::Valid(upper)) => (nums.len() as i32) - upper,
            _ => { -1 }
        };
        // println!("========> Start <=========");

        while left.next_index() < Index::InvalidOverflow && left.sum < target {
            left.move_next();
        }

        // Only left half result
        if left.sum == target {
            result.0 = left.index;
            value = result_value(&(left.index, right.index));
        }
        // Reduce one element so that the sum is lesser than target
        left.move_prev();
        // println!("After left loop, left:{} right:{} result:{:?}", left, right, result);

        // Decrement one index
        while right.next_index() > Index::InvalidUnderflow && right.next_index() > left.index {
            right.move_next();
            // println!("Inside loop, left:{} right:{} result:{:?}", left, right, result);

            match target.cmp(&(left.sum + right.sum)) {
                Ordering::Equal => {
                    // println!("Result equal");
                    let may_be_result = (left.index, right.index);
                    if result_value(&result) <= 0 ||
                        (result_value(&may_be_result) > 0 && result_value(&may_be_result) < result_value(&result)) {
                        result = may_be_result;
                    }
                    value = cmp::min(value, result_value(&(left.index, right.index)));
                    if left.index == Index::InvalidUnderflow { break; }
                }
                Ordering::Less => {
                    // println!("Result lesser, {} < {}", target, left.sum + right.sum);
                    if left.index != Index::InvalidOverflow && left.index != Index::InvalidUnderflow {
                        left.move_prev();
                        right.move_prev();
                    }
                }
                Ordering::Greater => {}
            }
        }
        // println!("result is {:?}", result);
        if value == i32::MAX { -1 } else { value }
    }
}

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(vec![5, 2, 3, 1, 1], 5), 1);
    assert_eq!(Solution::min_operations(vec![1, 1], 3), -1);
    assert_eq!(Solution::min_operations(vec![3, 2, 20, 1, 1, 3], 10), 5);
    assert_eq!(Solution::min_operations(vec![5, 6, 7, 8, 9], 4), -1);
    assert_eq!(Solution::min_operations(vec![1, 1, 4, 2, 3], 5), 2);
}

#[test]
fn test_min_operations_1() {
    assert_eq!(Solution::min_operations(vec![1, ], 3), -1);
}