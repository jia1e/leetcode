/*
 * @lc app=leetcode id=145 lang=rust
 *
 * [145] Binary Tree Postorder Traversal
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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut elements = vec![];
        Self::traversal(root, &mut elements);
        elements
    }

    fn traversal(node: Option<Rc<RefCell<TreeNode>>>, elements: &mut Vec<i32>) {
        if let Some(node) = node {
            Self::traversal(node.borrow_mut().left.take(), elements);
            Self::traversal(node.borrow_mut().right.take(), elements);
            elements.push(node.borrow().val);
        }
    }
}
// @lc code=end

pub struct Solution;
use crate::common::binary_tree::TreeNode;

#[test]
fn test() {
    use crate::binary_tree;

    let cases = [
        (binary_tree!(1, null, 2, 3), vec![3, 2, 1]),
        (binary_tree!(3, 1, 2), vec![1, 2, 3]),
        (binary_tree!(1, 2, 3, 4, 5, 6, 7), vec![4, 5, 2, 6, 7, 3, 1]),
    ];

    for (input, expected) in cases {
        assert_eq!(Solution::postorder_traversal(input), expected);
    }
}
