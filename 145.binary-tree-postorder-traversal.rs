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
    use crate::common::binary_tree::from_iter;
    let cases = [
        (
            vec![Some(1), None, Some(2), None, None, Some(3), None],
            vec![3, 2, 1],
        ),
        (vec![Some(3), Some(1), Some(2)], vec![1, 2, 3]),
        (
            vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7),
            ],
            vec![4, 5, 2, 6, 7, 3, 1],
        ),
    ];

    for (input, expected) in cases {
        let root = from_iter(input);
        let output = Solution::postorder_traversal(root);
        assert_eq!(output, expected);
    }
}
