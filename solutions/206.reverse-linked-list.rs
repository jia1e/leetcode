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

    pub fn reverse_list_2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut tail: Option<Box<ListNode>> = None;

        while let Some(mut node) = head {
            let next_head = std::mem::replace(&mut node.next, tail);
            tail = Some(node);
            head = next_head
        }

        tail
    }
}
// @lc code=end

pub struct Solution;

use crate::common::linked_list::ListNode;

#[test]
fn test() {
    use crate::linked_list;
    let cases = [
        (linked_list![], linked_list![]),
        (linked_list![1], linked_list![1]),
        (linked_list![1, 2], linked_list![2, 1]),
        (linked_list![1, 2, 3, 4, 5], linked_list![5, 4, 3, 2, 1]),
        (linked_list![1, 1], linked_list![1, 1]),
        (
            linked_list![1, 2, 3, 4, 5, 0],
            linked_list![0, 5, 4, 3, 2, 1],
        ),
    ];

    for (head, expected) in cases {
        assert_eq!(Solution::reverse_list(head.clone()), expected);
        assert_eq!(Solution::reverse_list_2(head.clone()), expected);
    }
}
