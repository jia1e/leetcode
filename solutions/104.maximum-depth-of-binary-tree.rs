/*
 * @lc app=leetcode id=104 lang=rust
 *
 * [104] Maximum Depth of Binary Tree
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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut depth = 0;

        let mut nodes = VecDeque::new();
        nodes.push_back(root);

        while !nodes.is_empty() {
            let len = nodes.len();

            depth += 1;
            for _ in 0..len {
                if let Some(node) = nodes.pop_front() {
                    let mut node = node.as_ref().unwrap().borrow_mut();
                    if node.left.is_some() {
                        nodes.push_back(node.left.take());
                    }

                    if node.right.is_some() {
                        nodes.push_back(node.right.take());
                    }
                }
            }
        }

        depth
    }

    pub fn max_depth_2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root == None {
            0
        } else {
            let mut node = root.as_ref().unwrap().borrow_mut();

            1 + Self::max_depth_2(node.left.take()).max(Self::max_depth_2(node.right.take()))
        }
    }
}
// @lc code=end

pub struct Solution;
use crate::shared::binary_tree::TreeNode;

#[test]
fn test() {
    use crate::binary_tree;
    let cases = [
        (binary_tree!(3, 9, 20, null, null, 15, 7), 3),
        (binary_tree!(), 0),
        (binary_tree!(1), 1),
        (binary_tree!(1, 2), 2),
        (binary_tree!(1, 2, 3), 2),
    ];

    for (input, expected) in cases {
        assert_eq!(Solution::max_depth(input), expected);
    }

    let cases = [
        (binary_tree!(3, 9, 20, null, null, 15, 7), 3),
        (binary_tree!(), 0),
        (binary_tree!(1), 1),
        (binary_tree!(1, 2), 2),
        (binary_tree!(1, 2, 3), 2),
    ];

    for (input, expected) in cases {
        assert_eq!(Solution::max_depth_2(input), expected);
    }
}
