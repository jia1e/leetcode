/*
 * @lc app=leetcode id=637 lang=rust
 *
 * [637] Average of Levels in Binary Tree
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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut avarages = vec![];
        if root.is_none() {
            return avarages;
        }

        let mut nodes = VecDeque::new();
        nodes.push_back(root);

        loop {
            let mut sum = 0_f64;
            let len = nodes.len();

            for _ in 0..len {
                if let Some(node) = nodes.pop_front() {
                    let mut node = node.as_ref().unwrap().borrow_mut();

                    sum += node.val as f64;

                    if node.left.is_some() {
                        nodes.push_back(node.left.take());
                    }

                    if node.right.is_some() {
                        nodes.push_back(node.right.take());
                    }
                }
            }

            avarages.push(sum / len as f64);

            if nodes.is_empty() {
                break;
            }
        }

        avarages
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
        vec![3.0, 14.5, 11.0],
    )];

    for (input, avarages) in cases {
        let root = from_iter(input);
        assert_eq!(Solution::average_of_levels(root), avarages);
    }
}
