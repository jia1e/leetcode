/*
 * @lc app=leetcode id=110 lang=rust
 *
 * [110] Balanced Binary Tree
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
use std::rc::Rc;
impl Solution {
    pub fn get_tree_height(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if node.is_none() {
            return 0;
        }
        let mut node = node.as_ref().unwrap().borrow_mut();

        let left_height = Self::get_tree_height(node.left.take());
        if left_height == -1 {
            return -1;
        }
        let right_height = Self::get_tree_height(node.right.take());
        if right_height == -1 {
            return -1;
        }

        if (left_height - right_height).abs() > 1 {
            -1
        } else {
            left_height.max(right_height) + 1
        }
    }

    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::get_tree_height(root) != -1
    }
}
// @lc code=end

pub struct Solution;
use crate::shared::binary_tree::TreeNode;

#[test]
fn test() {
    let cases = [
        (binary_tree!(3, 9, 20, null, null, 15, 7), true),
        (binary_tree!(1, 2, 2, 3, 3, null, null, 4, 4), false),
    ];

    for (root, expected) in cases {
        assert_eq!(Solution::is_balanced(root), expected);
    }
}
