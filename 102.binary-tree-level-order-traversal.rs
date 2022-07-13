/*
 * @lc app=leetcode id=102 lang=rust
 *
 * [102] Binary Tree Level Order Traversal
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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut layers = vec![];

        if root.is_none() {
            return layers;
        }

        let mut nodes = VecDeque::new();
        nodes.push_back(root);

        while !nodes.is_empty() {
            let len = nodes.len();
            let mut layer = vec![];
            for _ in 0..len {
                if let Some(Some(node)) = nodes.pop_front() {
                    let mut node = node.borrow_mut();
                    layer.push(node.val);
                    let left = node.left.take();
                    let right = node.right.take();
                    if left.is_some() {
                        nodes.push_back(left);
                    }
                    if right.is_some() {
                        nodes.push_back(right);
                    }
                }
            }
            layers.push(layer);
        }

        layers
    }

    pub fn level_order_2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut layers = vec![];

        if root.is_none() {
            return layers;
        }
        layers.push(Vec::new());
        Self::traversal(&root, 0, &mut layers);
        layers
    }

    fn traversal(node: &Option<Rc<RefCell<TreeNode>>>, depth: usize, layers: &mut Vec<Vec<i32>>) {
        if let Some(node) = node.as_ref() {
            let node = node.borrow();
            layers[depth].push(node.val);

            if (node.left.is_some() || node.right.is_some()) && layers.len() < depth + 2 {
                layers.push(Vec::new());
            }
            if node.left.is_some() {
                Self::traversal(&node.left, depth + 1, layers);
            }
            if node.right.is_some() {
                Self::traversal(&node.right, depth + 1, layers);
            }
        }
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
        vec![vec![3], vec![9, 20], vec![15, 7]],
    )];

    for (input, expected) in cases {
        let root = from_iter(input.clone());
        assert_eq!(Solution::level_order(root), expected);

        let root = from_iter(input.clone());
        assert_eq!(Solution::level_order_2(root), expected);
    }
}
