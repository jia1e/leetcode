/*
 * @lc app=leetcode id=2487 lang=rust
 *
 * [2487] Remove Nodes From Linked List
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
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode::new(i32::MAX)));
        dummy_head.as_mut().unwrap().next = head;
        Self::remove(&mut dummy_head);
        dummy_head.take().unwrap().next
    }

    fn remove(head: &mut Option<Box<ListNode>>) {
        if head.as_ref().unwrap().next.is_some() {
            Self::remove(&mut head.as_mut().unwrap().next)
        }

        if head.as_ref().unwrap().next.is_some() {
            if head.as_ref().unwrap().val < head.as_ref().unwrap().next.as_ref().unwrap().val {
                let next = head.as_mut().unwrap().next.take().unwrap();
                head.replace(next);
            }
        }
    }
}
// @lc code=end

pub struct Solution;
use crate::shared::linked_list::ListNode;

#[test]
fn test() {
    let cases = [
        (linked_list!(5, 2, 13, 3, 8), linked_list!(13, 8)),
        (linked_list!(1, 1, 1, 1), linked_list!(1, 1, 1, 1)),
        (
            linked_list!(10, 11, 12, 9, 9, 8, 9, 7, 6),
            linked_list!(12, 9, 9, 9, 7, 6),
        ),
    ];

    for (input, output) in cases {
        assert_eq!(Solution::remove_nodes(input), output)
    }
}
