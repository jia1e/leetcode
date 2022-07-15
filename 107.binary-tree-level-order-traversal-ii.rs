/*
 * @lc app=leetcode id=107 lang=rust
 *
 * [107] Binary Tree Level Order Traversal II
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
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut layers = vec![];
        if root.is_none() {
            return layers;
        }
        let mut nodes = VecDeque::new();
        nodes.push_back(root);

        loop {
            let len = nodes.len();

            let mut layer = vec![];
            for _ in 0..len {
                if let Some(node) = nodes.pop_front() {
                    let mut node = node.as_ref().unwrap().borrow_mut();

                    if node.left.is_some() {
                        nodes.push_back(node.left.take());
                    }

                    if node.right.is_some() {
                        nodes.push_back(node.right.take());
                    }

                    layer.push(node.val);
                }
            }

            layers.push(layer);

            if nodes.is_empty() {
                break;
            }
        }

        layers.reverse();

        layers
    }
}
// @lc code=end

pub struct Solution;

use crate::common::binary_tree::TreeNode;

#[test]
fn test() {
    use crate::common::binary_tree::from_iter;
    let cases = [(
        vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
        vec![vec![15, 7], vec![9, 20], vec![3]],
    )];

    for (input, expected) in cases {
        let root = from_iter(input.clone());
        assert_eq!(Solution::level_order_bottom(root), expected);
    }
}
