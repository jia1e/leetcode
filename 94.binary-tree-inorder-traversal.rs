/*
 * @lc app=leetcode id=94 lang=rust
 *
 * [94] Binary Tree Inorder Traversal
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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut elements = vec![];
        let mut nodes = vec![(root, vec![])];

        while let Some((node, mut next)) = nodes.pop() {
            if let Some(node) = node.as_ref() {
                let mut node = node.borrow_mut();
                match (node.left.take(), node.right.take(), node.val) {
                    (Some(left), Some(right), val) => {
                        nodes.push((Some(right), next));
                        nodes.push((Some(left), vec![val]));
                    }
                    (Some(left), None, val) => {
                        next.push(val);
                        nodes.push((Some(left), next));
                    }
                    (None, Some(right), val) => {
                        elements.push(val);
                        nodes.push((Some(right), next));
                    }
                    (None, None, val) => {
                        elements.push(val);
                        while let Some(val) = next.pop() {
                            elements.push(val)
                        }
                    }
                }
            }
        }
        elements
    }

    pub fn inorder_traversal_2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut elements = vec![];
        Self::traversal(root, &mut elements);
        elements
    }

    pub fn traversal(node: Option<Rc<RefCell<TreeNode>>>, elements: &mut Vec<i32>) {
        if let Some(node) = node {
            Self::traversal(node.borrow_mut().left.take(), elements);
            elements.push(node.borrow().val);
            Self::traversal(node.borrow_mut().right.take(), elements);
        }
    }
}
// @lc code=end

pub struct Solution;
use crate::common::binary_tree::TreeNode;

impl Solution {
    pub fn inorder_traversal_3(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut node = root;
        let mut res: Vec<i32> = vec![];
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];

        while !stack.is_empty() || node.is_some() {
            while let Some(n) = node {
                node = n.borrow_mut().left.take();
                stack.push(Some(n));
            }
            if let Some(Some(n)) = stack.pop() {
                res.push(n.borrow().val);
                node = n.borrow_mut().right.take();
            }
        }
        return res;
    }
}

#[test]
fn test() {
    use crate::common::binary_tree::from_iter;
    let cases = [
        (
            vec![Some(1), None, Some(2), None, None, Some(3), None],
            vec![1, 3, 2],
        ),
        (
            vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7),
            ],
            vec![4, 2, 5, 1, 6, 3, 7],
        ),
        (vec![Some(2), Some(3), None, Some(1)], vec![1, 3, 2]),
        (vec![Some(1), Some(4), Some(3), Some(2)], vec![2, 4, 1, 3]),
    ];

    for (input, expected) in cases {
        let root = from_iter(input.clone());
        let output = Solution::inorder_traversal(root);
        assert_eq!(output, expected);

        let root = from_iter(input.clone());
        let output = Solution::inorder_traversal_2(root);
        assert_eq!(output, expected);

        let root = from_iter(input.clone());
        let output = Solution::inorder_traversal_3(root);
        assert_eq!(output, expected);
    }
}
