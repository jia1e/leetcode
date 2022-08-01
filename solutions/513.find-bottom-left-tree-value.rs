/*
 * @lc app=leetcode id=513 lang=rust
 *
 * [513] Find Bottom Left Tree Value
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::from([root]);

        let mut value = 0;
        while !queue.is_empty() {
            let n = queue.len();

            for _ in 0..n {
                if let Some(node) = queue.pop_front() {
                    let mut node = node.as_ref().unwrap().borrow_mut();
                    value = node.val;
                    if node.right.is_some() {
                        queue.push_back(node.right.take())
                    }
                    if node.left.is_some() {
                        queue.push_back(node.left.take())
                    }
                }
            }
        }

        value
    }
}
// @lc code=end

pub struct Solution;
use crate::shared::binary_tree::TreeNode;

#[test]
fn test() {
    let cases = [
        (binary_tree!(2, 1, 3), 1),
        (binary_tree!(1, 2, 3, 4, null, 5, 6, null, null, 7), 7),
    ];

    for (root, expected) in cases {
        assert_eq!(Solution::find_bottom_left_value(root), expected);
    }
}
