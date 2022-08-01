/*
 * @lc app=leetcode id=112 lang=rust
 *
 * [112] Path Sum
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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }

        let mut node = root.as_ref().unwrap().borrow_mut();

        match (node.left.take(), node.right.take()) {
            (None, None) => node.val == target_sum,
            (left, None) => Self::has_path_sum(left, target_sum - node.val),
            (None, right) => Self::has_path_sum(right, target_sum - node.val),
            (left, right) => {
                Self::has_path_sum(left, target_sum - node.val)
                    || Self::has_path_sum(right, target_sum - node.val)
            }
        }
    }
}
// @lc code=end

pub struct Solution;
use crate::shared::binary_tree::TreeNode;

#[test]
fn test() {
    let cases = [
        (binary_tree!(1, 2, 3), 5, false),
        (binary_tree!(), 0, false),
        (
            binary_tree!(5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1),
            22,
            true,
        ),
    ];

    for (root, target_sum, expected) in cases {
        assert_eq!(Solution::has_path_sum(root, target_sum), expected);
    }
}
