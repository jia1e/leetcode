/*
 * @lc app=leetcode id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode::new(i32::MIN);
        let mut tail = &mut dummy_head;

        let mut l1 = list1;
        let mut l2 = list2;

        while l1.is_some() && l2.is_some() {
            if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
                let l1_next = l1.as_mut().unwrap().next.take();
                tail.next = l1;
                l1 = l1_next;
            } else {
                let l2_next = l2.as_mut().unwrap().next.take();
                tail.next = l2;
                l2 = l2_next;
            }

            tail = tail.next.as_mut().unwrap();
        }

        if l1.is_some() {
            tail.next = l1;
        } else if l2.is_some() {
            tail.next = l2;
        }

        dummy_head.next
    }
}
// @lc code=end

pub struct Solution;

use crate::shared::linked_list::ListNode;

#[test]
fn test() {
    let l1 = linked_list!(1, 2, 4);
    let l2 = linked_list!(1, 3, 4);

    assert_eq!(
        Solution::merge_two_lists(l1, l2),
        linked_list!(1, 1, 2, 3, 4, 4)
    );
}
