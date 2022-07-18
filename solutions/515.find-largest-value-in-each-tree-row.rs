/*
 * @lc app=leetcode id=515 lang=rust
 *
 * [515] Find Largest Value in Each Tree Row
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
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut largest_values = vec![];

        if root.is_none() {
            return largest_values;
        }

        let mut nodes = VecDeque::new();
        nodes.push_back(root);

        while !nodes.is_empty() {
            largest_values.push(i32::MIN);
            let largest = largest_values.last_mut().unwrap();
            let len = nodes.len();

            for _ in 0..len {
                if let Some(node) = nodes.pop_front() {
                    let mut node = node.as_ref().unwrap().borrow_mut();
                    if node.val > *largest {
                        *largest = node.val;
                    }

                    if node.left.is_some() {
                        nodes.push_back(node.left.take());
                    }

                    if node.right.is_some() {
                        nodes.push_back(node.right.take());
                    }
                }
            }
        }

        largest_values
    }
}
// @lc code=end

pub struct Solution;
use crate::shared::binary_tree::TreeNode;

#[test]
fn test() {
    use crate::binary_tree;
    let cases = [(
        binary_tree!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15),
        vec![1, 3, 7, 15],
    )];

    for (input, expected) in cases {
        assert_eq!(Solution::largest_values(input), expected);
    }
}
