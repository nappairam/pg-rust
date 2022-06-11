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

// use std::mem;
//
// struct Solution;
//
// // Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode {
//             next: None,
//             val,
//         }
//     }
// }
//
// impl Solution {
//     pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         let mut head = ListNode::new(-1);
//         // let mut tail = &head.next;
//         // let mut list1 = list1;
//         // let mut list2 = list2;
//         //
//         // while list1 != None && list2 != None {
//         //     match (&mut list1, &mut list2) {
//         //         (None, None) => println!("Both are none"),
//         //         (Some(node1), Some(node2)) => {
//         //             // if node1.val < node2.val {
//         //             //     let node1_next = mem::take(&mut node1.next);
//         //             //     let removed_node = node1;
//         //             //     *node1 = node1_next;
//         //             //     head.next = removed_node;
//         //             // }
//         //         },
//         //         (_, _) => println!("Catch all case"),
//         //     }
//         // }
//         head.next
//     }
// }