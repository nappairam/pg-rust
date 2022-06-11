//
//  https://leetcode.com/problems/merge-k-sorted-lists/
//  23. Merge k Sorted Lists
// You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.
//
// Merge all the linked-lists into one sorted linked-list and return it.
//
//
//
// Example 1:
//
// Input: lists = [[1,4,5],[1,3,4],[2,6]]
// Output: [1,1,2,3,4,4,5,6]
// Explanation: The linked-lists are:
// [
//   1->4->5,
//   1->3->4,
//   2->6
// ]
// merging them into one sorted list:
// 1->1->2->3->4->4->5->6

// Second problem:
// https://leetcode.com/problems/reverse-nodes-in-k-group/:w
// 25. Reverse Nodes in k-Group
//
// Given the head of a linked list, reverse the nodes of the list k at a time, and return the modified list.
//
// k is a positive integer and is less than or equal to the length of the linked list.
// If the number of nodes is not a multiple of k then left-out nodes, in the end, should remain as it is.
//
// You may not alter the values in the list's nodes, only nodes themselves may be changed.

use std::fmt;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from_list(arr: Vec<i32>) -> Option<Box<Self>> {
        if arr.len() == 0 {
            return None;
        }
        let mut list: Box<ListNode> = Box::new(ListNode::new(arr[0]));

        for elem in arr[1..].iter() {
            list.push(*elem);
        }
        Some(list)
    }

    fn to_list(&self) -> Vec<i32> {
        let mut vector = vec![];
        vector.push(self.val);
        let mut temp: &Option<Box<ListNode>> = &self.next;
        while !temp.is_none() {
            let next_node = temp.as_ref().unwrap();
            temp = &next_node.next;
            vector.push(next_node.val);
        }
        vector
    }

    fn push(&mut self, val: i32) -> () {
        let mut temp: &mut Option<Box<ListNode>> = &mut self.next;
        let new_node = Box::new(ListNode::new(val));
        while !temp.is_none() {
            temp = &mut temp.as_mut().unwrap().next;
        }
        let _ = std::mem::replace(temp, Some(new_node));
    }
}

impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ {}, ", &self.val).unwrap_or_default();
        let mut temp: &Option<Box<ListNode>> = &self.next;
        while !temp.is_none() {
            let next_node = temp.as_ref().unwrap();
            temp = &next_node.next;
            write!(f, "{}, ", next_node.val).unwrap_or_default();
        }
        write!(f, "]")
    }
}

struct Solution;

impl Solution {
    fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.len() == 0 {
            return None;
        }
        let mut merged_list: Box<ListNode> = Box::new(ListNode::new(i32::MIN));
        let mut temp_list: Vec<i32> = vec![i32::MAX; lists.len()];
        'outer: loop {
            for (index, list) in lists.iter_mut().enumerate() {
                if temp_list[index] == i32::MAX {
                    if let Some(node) = list {
                        temp_list[index] = node.val;
                        *list = std::mem::take(&mut node.next);
                    }
                }
            }
            let minimum = temp_list.iter().enumerate().fold(
                (usize::MAX, i32::MAX),
                |(acc_ind, acc_val), (cur_ind, &cur_val)| {
                    if cur_val < acc_val {
                        (cur_ind, cur_val)
                    } else {
                        (acc_ind, acc_val)
                    }
                },
            );
            if minimum.0 != usize::MAX {
                merged_list.push(minimum.1);
                temp_list[minimum.0] = i32::MAX;
            }

            // println!("TempList:{:?} Minimum:{:?}", temp_list, minimum);
            if temp_list.iter().all(|&i| i == i32::MAX) {
                if minimum.0 != usize::MAX {
                    while let Some(node) = &mut lists[minimum.0] {
                        merged_list.push(node.val);
                        lists[minimum.0] = std::mem::take(&mut node.next)
                    }
                }
                break 'outer;
            }
        }
        // println!("{}", merged_list);
        merged_list.next
    }

    fn reverse_k_group(_head: Option<Box<ListNode>>, _k: i32) -> Option<Box<ListNode>> {
        unimplemented!()
    }
}

#[test]
fn test_longest_parenthesis() {
    let list = ListNode::from_list(vec![1, 2, 3, 4]);
    let list1 = ListNode::from_list(vec![3, 4, 5]);
    let list2 = ListNode::from_list(vec![-1, 3, 5]);
    let list3 = ListNode::from_list(vec![10, 14, 17, 19]);
    let list4 = ListNode::from_list(vec![1, 4, 9]);
    let list_array = vec![list, list1, list2, list3, list4];

    let sol = Solution::merge_k_lists(list_array).unwrap().to_list();

    assert_eq!(
        sol,
        vec![-1, 1, 1, 2, 3, 3, 3, 4, 4, 4, 5, 5, 9, 10, 14, 17, 19]
    );
    assert_eq!(Solution::merge_k_lists(vec![]), None);
    assert_eq!(
        Solution::merge_k_lists(vec![ListNode::from_list(vec![])]),
        None
    );
    assert_eq!(
        Solution::merge_k_lists(vec![
            ListNode::from_list(vec![]),
            ListNode::from_list(vec![])
        ]),
        None
    );
    assert_eq!(
        Solution::reverse_k_group(ListNode::from_list(vec![]), 0),
        None
    );
    println!("Merged sorted lists success")
}
