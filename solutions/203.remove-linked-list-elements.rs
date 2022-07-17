/*
 * @lc app=leetcode id=203 lang=rust
 *
 * [203] Remove Linked List Elements
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut walker = &mut head;
        while walker.is_some() {
            if walker.as_ref().unwrap().val == val {
                *walker = walker.take().unwrap().next;
            } else {
                walker = &mut walker.as_mut().unwrap().next;
            }
        }
        head
    }
}
// @lc code=end

use crate::common::list::ListNode;

pub struct Solution;

#[test]
fn test() {
    use crate::common::list::{from_iter, into_vec};
    let cases = [
        (vec![], 0, vec![]),
        (vec![1], 0, vec![1]),
        (vec![1, 2, 3], 1, vec![2, 3]),
        (vec![1, 2, 3, 4, 4], 4, vec![1, 2, 3]),
    ];

    for (nums, val, output) in cases {
        assert_eq!(
            into_vec(Solution::remove_elements(from_iter(nums), val)),
            output
        )
    }
}
