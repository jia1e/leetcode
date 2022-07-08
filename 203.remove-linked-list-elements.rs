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
}

pub struct Solution;

fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Option<Box<ListNode>> {
    let mut dummy_head = ListNode::new(0);
    let mut pre = &mut dummy_head;
    for val in iter {
        pre.next = Some(Box::new(ListNode::new(val)));
        pre = pre.next.as_mut().unwrap();
    }
    dummy_head.next
}

fn into_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = vec![];
    let mut node = &head;
    while node.is_some() {
        result.push(node.as_ref().unwrap().val);
        node = &node.as_ref().unwrap().next
    }
    result
}

#[test]
fn test() {
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
