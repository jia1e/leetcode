/*
 * @lc app=leetcode id=226 lang=rust
 *
 * [226] Invert Binary Tree
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
    pub fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::invert(&mut root);
        root
    }

    fn invert(node: &mut Option<Rc<RefCell<TreeNode>>>) {
        if node.is_some() {
            let mut node = node.as_ref().unwrap().borrow_mut();
            let left = node.left.take();
            let right = node.right.take();
            node.right = left;
            node.left = right;

            Self::invert(&mut node.left);
            Self::invert(&mut node.right);
        }
    }
}
// @lc code=end

pub struct Solution;
use crate::common::binary_tree::TreeNode;

#[test]
fn test() {
    use crate::binary_tree;

    let cases = [(
        binary_tree!(4, 2, 7, 1, 3, 6, 9),
        binary_tree!(4, 7, 2, 9, 6, 3, 1),
    )];

    for (input, output) in cases {
        assert_eq!(Solution::invert_tree(input), output);
    }
}
