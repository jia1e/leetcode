/*
 * @lc app=leetcode id=257 lang=rust
 *
 * [257] Binary Tree Paths
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
    fn traversal(
        node: Option<Rc<RefCell<TreeNode>>>,
        parent_path: &mut String,
        paths: &mut Vec<String>,
    ) {
        let mut node = node.as_ref().unwrap().borrow_mut();

        if !parent_path.is_empty() {
            parent_path.push_str(&"->")
        }

        parent_path.push_str(node.val.to_string().as_str());

        let (left, right) = (node.left.take(), node.right.take());

        if left.is_none() && right.is_none() {
            paths.push(parent_path.clone())
        } else {
            if left.is_some() {
                Self::traversal(left, &mut parent_path.clone(), paths);
            }

            if right.is_some() {
                Self::traversal(right, &mut parent_path.clone(), paths);
            }
        }
    }

    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut paths = vec![];
        if root.is_some() {
            Self::traversal(root, &mut String::new(), &mut paths);
        }
        paths
    }
}
// @lc code=end

pub struct Solution;
use crate::shared::binary_tree::TreeNode;

#[test]
fn test() {
    let cases = [
        (binary_tree!(1, 2, 3, null, 5), vec!["1->2->5", "1->3"]),
        (binary_tree!(1), vec!["1"]),
    ];

    for (root, expected) in cases {
        let paths = Solution::binary_tree_paths(root);
        assert_eq!(
            paths,
            expected.iter().map(|s| s.to_string()).collect::<Vec<_>>()
        )
    }
}
