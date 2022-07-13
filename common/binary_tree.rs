use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    collections::VecDeque,
    rc::Rc,
};

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[allow(dead_code)]
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[allow(dead_code)]
pub fn from_iter<T: IntoIterator<Item = Option<i32>>>(iter: T) -> Option<Rc<RefCell<TreeNode>>> {
    let mut nodes = vec![];

    for item in iter {
        if let Some(val) = item {
            nodes.push(Some(Rc::new(RefCell::new(TreeNode::new(val)))));
        } else {
            nodes.push(None)
        }
    }

    let mut index = nodes.len() - 1;

    while !nodes.is_empty() {
        if index == 0 {
            return nodes.pop().unwrap();
        }

        let node = nodes.pop().unwrap().take();

        if let Some(rc) = &nodes[(index - 1) / 2] {
            if index % 2 == 1 {
                rc.as_ref().borrow_mut().left = node;
            } else {
                rc.as_ref().borrow_mut().right = node;
            }
        }

        index -= 1;
    }

    None
}

#[allow(dead_code)]
pub fn to_vec(tree: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut v = vec![];
    let mut node = tree;
    let mut layer = 0;

    v
}

#[test]
fn test() {
    let tree = from_iter([
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
    ]);
    println!("{:#?}", tree);
}
