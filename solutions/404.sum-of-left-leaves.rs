/*
 * @lc app=leetcode id=404 lang=rust
 *
 * [404] Sum of Left Leaves
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
    fn traversal(node: Option<Rc<RefCell<TreeNode>>>, is_left: bool, sum: &mut i32) {
        if node.is_none() {
            return;
        }

        let mut node = node.as_ref().unwrap().borrow_mut();

        match (node.left.take(), node.right.take()) {
            (None, None) => {
                if is_left {
                    *sum += node.val
                }
            }
            (left, right) => {
                Self::traversal(left, true, sum);
                Self::traversal(right, false, sum);
            }
        }
    }

    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        Self::traversal(root, false, &mut sum);
        sum
    }
}
// @lc code=end

pub struct Solution {}
use crate::shared::binary_tree::TreeNode;

#[test]
fn test() {
    let cases = [
        (binary_tree!(3, 9, 20, null, null, 15, 7), 24),
        (binary_tree!(1), 0),
    ];

    for (root, expected) in cases {
        assert_eq!(Solution::sum_of_left_leaves(root), expected);
    }
}
