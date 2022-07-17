/*
 * @lc app=leetcode id=101 lang=rust
 *
 * [101] Symmetric Tree
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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        let mut node = root.as_ref().unwrap().borrow_mut();
        let (mut left_queue, mut right_queue) = (VecDeque::new(), VecDeque::new());
        left_queue.push_back(node.left.take());
        right_queue.push_back(node.right.take());

        loop {
            let len = left_queue.len();

            for _ in 0..len {
                match (
                    left_queue.pop_front().unwrap(),
                    right_queue.pop_front().unwrap(),
                ) {
                    (Some(left), Some(right)) => {
                        let (mut left, mut right) = (left.borrow_mut(), right.borrow_mut());
                        if left.val == right.val {
                            left_queue.push_back(left.left.take());
                            left_queue.push_back(left.right.take());

                            right_queue.push_back(right.right.take());
                            right_queue.push_back(right.left.take());
                        } else {
                            return false;
                        }
                    }
                    (None, None) => continue,
                    _ => return false,
                }
            }

            if left_queue.len() == 0 {
                break;
            }
        }

        true
    }
}

// @lc code=end

pub struct Solution;
use crate::common::binary_tree::TreeNode;

#[test]
fn test() {
    use crate::binary_tree;
    let cases = [
        (binary_tree!(1, 2, 2, 3, 4, 4, 3), true),
        (binary_tree!(1, 2, 2, null, 3, null, 3), false),
    ];

    for (root, expected) in cases {
        assert_eq!(Solution::is_symmetric(root), expected);
    }
}
