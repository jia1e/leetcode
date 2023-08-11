/*
 * @lc app=leetcode id=23 lang=rust
 *
 * [23] Merge k Sorted Lists
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
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode::new(i32::MIN);
        let mut tail = &mut dummy_head;
        let mut remain = lists.len();

        while remain > 0 {
            lists.sort_by(|a, b| {
                if a.is_none() {
                    return std::cmp::Ordering::Greater;
                }
                if b.is_none() {
                    return std::cmp::Ordering::Less;
                }

                return a.as_ref().unwrap().val.cmp(&b.as_ref().unwrap().val);
            });

            if lists[0].is_none() {
                break;
            }

            let v = lists[0].as_ref().unwrap().val;

            for i in 0..remain {
                let mut lh = lists[i].take();

                while lh.is_some() && lh.as_ref().unwrap().val == v {
                    tail.next = lh;
                    lh = tail.next.as_mut().unwrap().next.take();
                    tail = tail.next.as_mut().unwrap();
                }

                if lh.is_none() {
                    remain -= 1;
                }

                lists[i] = lh;
            }
        }

        dummy_head.next
    }
}
// @lc code=end

pub struct Solution;
use crate::shared::linked_list::ListNode;

#[test]
fn test() {
    let lists = vec![
        linked_list!(),
        linked_list!(1, 2, 3, 4),
        linked_list!(5, 6, 7, 8),
        linked_list!(1, 5, 9),
    ];
    let merged = Solution::merge_k_lists(lists);
    assert_eq!(merged, linked_list!(1, 1, 2, 3, 4, 5, 5, 6, 7, 8, 9));
}

#[test]
fn test1() {
    let mut list = vec![Some(1), Some(2), None, None];

    list.sort_by(|a, b| {
        if a.is_none() {
            return std::cmp::Ordering::Greater;
        }
        if b.is_none() {
            return std::cmp::Ordering::Less;
        }
        return a.unwrap().cmp(b.as_ref().unwrap());
    });

    println!("{:?}", list);
}
