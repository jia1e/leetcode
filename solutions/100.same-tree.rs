/*
 * @lc app=leetcode id=100 lang=rust
 *
 * [100] Same Tree
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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut nodes = vec![p, q];

        while !nodes.is_empty() {
            match (nodes.pop().unwrap(), nodes.pop().unwrap()) {
                (Some(p), Some(q)) => {
                    let (mut p, mut q) = (p.borrow_mut(), q.borrow_mut());
                    if p.val != q.val {
                        return false;
                    }
                    nodes.push(p.left.take());
                    nodes.push(q.left.take());
                    nodes.push(p.right.take());
                    nodes.push(q.right.take());
                }
                (None, None) => continue,
                _ => return false,
            }
        }

        true
    }

    pub fn is_same_tree_2(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                let (mut p, mut q) = (p.borrow_mut(), q.borrow_mut());
                if p.val != q.val {
                    false
                } else {
                    Self::is_same_tree(p.left.take(), q.left.take())
                        && Self::is_same_tree(p.right.take(), q.right.take())
                }
            }
            (None, None) => true,
            _ => false,
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
        (binary_tree!(1, 2, 3), binary_tree!(1, 2, 3), true),
        (binary_tree!(1, 2), binary_tree!(1, null, 2), false),
    ];
    for (p, q, output) in cases {
        assert_eq!(Solution::is_same_tree(p, q), output);
    }

    let cases = [
        (binary_tree!(1, 2, 3), binary_tree!(1, 2, 3), true),
        (binary_tree!(1, 2), binary_tree!(1, null, 2), false),
    ];
    for (p, q, output) in cases {
        assert_eq!(Solution::is_same_tree_2(p, q), output);
    }
}
