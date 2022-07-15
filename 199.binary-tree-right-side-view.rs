/*
 * @lc app=leetcode id=199 lang=rust
 *
 * [199] Binary Tree Right Side View
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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut view = vec![];
        Self::traversal(root, &mut view, 1);
        view
    }

    fn traversal(node: Option<Rc<RefCell<TreeNode>>>, view: &mut Vec<i32>, depth: usize) {
        if let Some(node) = node {
            if view.len() < depth {
                view.push(node.borrow().val);
            }

            Self::traversal(node.borrow_mut().right.take(), view, depth + 1);
            Self::traversal(node.borrow_mut().left.take(), view, depth + 1);
        }
    }

    pub fn right_side_view_2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut view = vec![];

        if root.is_none() {
            return view;
        }

        let mut nodes = VecDeque::new();
        nodes.push_back(root);

        loop {
            let len = nodes.len();
            let mut found = false;
            for _ in 0..len {
                if let Some(node) = nodes.pop_front() {
                    let mut node = node.as_ref().unwrap().borrow_mut();

                    if node.right.is_some() {
                        nodes.push_back(node.right.take());
                    }
                    if node.left.is_some() {
                        nodes.push_back(node.left.take());
                    }

                    if !found {
                        view.push(node.val);
                        found = true;
                    }
                }
            }
            if nodes.is_empty() {
                break;
            }
        }

        view
    }
}
// @lc code=end

pub struct Solution;
use crate::common::binary_tree::TreeNode;

#[test]
fn test() {
    use crate::common::binary_tree::from_iter;

    let cases = [(
        vec![Some(1), Some(2), Some(3), None, Some(5), None, Some(4)],
        vec![1, 3, 4],
    )];

    for (input, expected) in cases {
        let root = from_iter(input);
        assert_eq!(Solution::right_side_view(root), expected);
    }
}
