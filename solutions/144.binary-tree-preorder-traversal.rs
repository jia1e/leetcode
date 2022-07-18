/*
 * @lc app=leetcode id=144 lang=rust
 *
 * [144] Binary Tree Preorder Traversal
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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut elements = vec![];
        Self::traversal(root, &mut elements);
        elements
    }

    fn traversal(node: Option<Rc<RefCell<TreeNode>>>, elements: &mut Vec<i32>) {
        if let Some(node) = node {
            elements.push(node.borrow().val);
            Self::traversal(node.borrow_mut().left.take(), elements);
            Self::traversal(node.borrow_mut().right.take(), elements);
        }
    }

    pub fn preorder_traversal_2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut elements = vec![];
        let mut nodes = vec![root];

        while !nodes.is_empty() {
            if let Some(node) = nodes.pop().unwrap() {
                elements.push(node.borrow().val);
                nodes.push(node.borrow_mut().right.take());
                nodes.push(node.borrow_mut().left.take());
            }
        }

        elements
    }
}
// @lc code=end

pub struct Solution;

use crate::shared::binary_tree::TreeNode;

#[test]
fn test() {
    use crate::binary_tree;

    let cases = [
        (binary_tree!(1, null, 2, 3), vec![1, 2, 3]),
        (binary_tree!(1, 2, 3, 4, 5, 6, 7), vec![1, 2, 4, 5, 3, 6, 7]),
    ];

    for (input, expected) in cases {
        let output = Solution::preorder_traversal(input);
        assert_eq!(output, expected);
    }

    let cases = [
        (binary_tree!(1, null, 2, 3), vec![1, 2, 3]),
        (binary_tree!(1, 2, 3, 4, 5, 6, 7), vec![1, 2, 4, 5, 3, 6, 7]),
    ];

    for (input, expected) in cases {
        let output = Solution::preorder_traversal_2(input);
        assert_eq!(output, expected);
    }
}
