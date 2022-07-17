/*
 * @lc app=leetcode id=222 lang=rust
 *
 * [222] Count Complete Tree Nodes
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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let mut left = node.borrow().left.clone();
                let mut right = node.borrow().right.clone();

                let (mut left_height, mut right_height) = (0, 0);

                while let Some(l) = left {
                    left = l.borrow().left.clone();
                    left_height += 1;
                }

                while let Some(r) = right {
                    right = r.borrow().right.clone();
                    right_height += 1;
                }

                if left_height == right_height {
                    (2 << left_height) - 1
                } else {
                    let mut node = node.borrow_mut();
                    1 + Self::count_nodes(node.left.take()) + Self::count_nodes(node.right.take())
                }
            }
            _ => 0,
        }
    }

    pub fn count_nodes_2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let mut node = node.borrow_mut();
                1 + Self::count_nodes(node.left.take()) + Self::count_nodes(node.right.take())
            }
            _ => 0,
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
        (binary_tree!(1, 2, 3, 4, 5, 6), 6),
        (binary_tree!(), 0),
        (binary_tree!(1), 1),
    ];

    for (root, count) in cases {
        assert_eq!(Solution::count_nodes(root), count);
    }
}
