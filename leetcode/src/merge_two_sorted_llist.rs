// 21. Merge Two Sorted Lists
// https://leetcode.com/problems/merge-two-sorted-lists/
//
// You are given the heads of two sorted linked lists list1 and list2.
// Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.
//
// Return the head of the merged linked list.
//
// Example 1:
// Input: list1 = [1,2,4], list2 = [1,3,4]
// Output: [1,1,2,3,4,4]
//
// Example 2:
// Input: list1 = [], list2 = []
// Output: []
//
// Example 3:
// Input: list1 = [], list2 = [0]
// Output: [0]
//
// Constraints:
// The number of nodes in both lists is in the range [0, 50].
// -100 <= Node.val <= 100
// Both list1 and list2 are sorted in non-decreasing order.

use std::mem;

struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }

    #[inline]
    fn from_list(nums: Vec<i32>) -> Option<Box<Self>> {
        let mut head = None;
        for &num in nums.iter().rev() {
            let mut node = Box::new(Self::new(num));
            mem::swap(&mut node.next, &mut head);
            head = Some(node);
        }
        head
    }
}

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(-1);
        let mut tail = &mut head.next;
        let mut list1 = list1;
        let mut list2 = list2;

        while list1 != None || list2 != None {
            match (list1.as_mut(), list2.as_mut()) {
                (Some(node1), Some(node2)) => {
                    // println!("Inside first case {:?} {:?}", node1, node2);
                    if node1.val < node2.val {
                        let mut node1_next = mem::take(&mut node1.next);
                        mem::swap(&mut node1_next, &mut list1);
                        mem::swap(&mut *tail, &mut node1_next);
                        tail = &mut tail.as_mut()?.next;
                    } else {
                        let mut node2_next = mem::take(&mut node2.next);
                        mem::swap(&mut node2_next, &mut list2);
                        mem::swap(&mut *tail, &mut node2_next);
                        tail = &mut tail.as_mut()?.next;
                    }
                }
                (Some(node1), None) => {
                    // println!("Inside second case {:?}", node1);
                    let mut node1_next = mem::take(&mut node1.next);
                    mem::swap(&mut node1_next, &mut list1);
                    mem::swap(&mut *tail, &mut node1_next);
                    tail = &mut tail.as_mut()?.next;
                }
                (None, Some(node2)) => {
                    // println!("Inside third case {:?}", node2);
                    let mut node2_next = mem::take(&mut node2.next);
                    mem::swap(&mut node2_next, &mut list2);
                    mem::swap(&mut *tail, &mut node2_next);
                    tail = &mut tail.as_mut()?.next;
                }
                (None, None) => println!("Both are none"),
            }
        }
        head.next
    }
}

#[test]
fn test_create_from_list() {
    let a = vec![1, 2, 3];
    println!("Node is {:?}", ListNode::from_list(a));
}

#[test]
fn test_merge_two_lists() {
    let a = ListNode::from_list(vec![1, 2, 4]);
    let b = ListNode::from_list(vec![1, 3, 4]);

    println!("Solution is {:?}", Solution::merge_two_lists(a, b));
}