/*
 * @lc app=leetcode id=206 lang=rust
 *
 * [206] Reverse Linked List
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
    fn reverse_node(
        head: Option<Box<ListNode>>,
        tail: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            let next_head = std::mem::replace(&mut node.next, tail);
            Self::reverse_node(next_head, Some(node))
        } else {
            tail
        }
    }

    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::reverse_node(head, None)
    }
}
// @lc code=end

pub struct Solution;

use crate::common::list::ListNode;

#[test]
fn test() {
    use crate::common::list::{from_iter, into_vec};
    let cases = [
        vec![],
        vec![1],
        vec![1, 2],
        vec![1, 2, 3, 4, 5],
        vec![1, 1],
        vec![1, 2, 3, 4, 5, 0],
    ];

    for input in cases {
        let mut expected = input.clone();
        expected.reverse();
        let output = Solution::reverse_list(from_iter(input));
        assert_eq!(into_vec(output), expected);
    }
}
