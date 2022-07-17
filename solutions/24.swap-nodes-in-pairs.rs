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

use crate::common::list::ListNode;

#[test]
fn test() {
    use crate::common::list::{from_iter, into_vec};

    let cases = [vec![1, 2, 3, 4, 5, 6, 7], vec![], vec![1]];

    for input in cases {
        let mut expected = input.clone();
        for i in 0..(expected.len() / 2) {
            expected.swap(2 * i, 2 * i + 1);
        }
        let head = from_iter(input.clone());
        assert_eq!(into_vec(Solution::swap_pairs(head)), expected);

        let head = from_iter(input.clone());
        assert_eq!(into_vec(Solution::swap_pairs_2(head)), expected);

        let head = from_iter(input.clone());
        assert_eq!(into_vec(Solution::swap_pairs_3(head)), expected);
    }
}
