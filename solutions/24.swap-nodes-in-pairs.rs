/*
 * @lc app=leetcode id=24 lang=rust
 *
 * [24] Swap Nodes in Pairs
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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.map(|mut head| match head.next {
            Some(mut node) => {
                head.next = Solution::swap_pairs(node.next);
                node.next = Some(head);
                node
            }
            None => head,
        })
    }

    pub fn swap_pairs_2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.and_then(|mut head| match head.next {
            Some(mut node) => {
                head.next = Solution::swap_pairs_2(node.next);
                node.next = Some(head);
                Some(node)
            }
            None => Some(head),
        })
    }

    pub fn swap_pairs_3(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = &mut head;
        while let Some(mut left) = cur
            .as_deref_mut()
            .and_then(|left| left.next.take())
            .and_then(|right| cur.replace(right))
        {
            let right = cur.as_deref_mut().unwrap();
            left.next = right.next.take();
            right.next.replace(left);
            cur = &mut right.next.as_deref_mut().unwrap().next;
        }
        head
    }
}
// @lc code=end

pub struct Solution;

use crate::shared::linked_list::ListNode;

#[test]
fn test() {
    use crate::linked_list;

    let cases = [
        (
            linked_list!(1, 2, 3, 4, 5, 6, 7),
            linked_list!(2, 1, 4, 3, 6, 5, 7),
        ),
        (
            linked_list!(1, 2, 3, 4, 5, 6),
            linked_list!(2, 1, 4, 3, 6, 5),
        ),
        (linked_list!(), linked_list!()),
        (linked_list!(1), linked_list!(1)),
    ];

    for (head, expected) in cases {
        assert_eq!(Solution::swap_pairs(head.clone()), expected);

        assert_eq!(Solution::swap_pairs_2(head.clone()), expected);

        assert_eq!(Solution::swap_pairs_3(head.clone()), expected);
    }
}
