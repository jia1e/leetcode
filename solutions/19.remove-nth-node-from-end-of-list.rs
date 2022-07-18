/*
 * @lc app=leetcode id=19 lang=rust
 *
 * [19] Remove Nth Node From End of List
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if n == 0 {
            return head;
        }

        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut len = 0;
        let mut tail = &dummy_head;
        while tail.as_ref().unwrap().next.is_some() {
            len += 1;
            tail = &tail.as_ref().unwrap().next;
        }
        let mut parent = &mut dummy_head;

        for _ in 0..len - n {
            parent = &mut parent.as_mut().unwrap().next;
        }

        parent.as_mut().unwrap().next = parent.as_mut().unwrap().next.as_mut().unwrap().next.take();

        dummy_head.unwrap().next
    }
}
// @lc code=end

pub struct Solution;

use crate::common::linked_list::ListNode;

#[test]
fn test() {
    use crate::linked_list;
    let cases = [(linked_list![1, 2, 3, 4, 5], 4, linked_list![1, 3, 4, 5])];

    for (head, n, expected) in cases {
        assert_eq!(Solution::remove_nth_from_end(head, n), expected)
    }
}
