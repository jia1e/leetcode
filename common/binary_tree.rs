use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
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
    let mut items: VecDeque<_> = iter.into_iter().collect();

    let mut nodes = vec![];
    let mut none_nodes = HashSet::new();
    let mut index = 0_usize;

    while !items.is_empty() {
        if none_nodes.contains(&index) {
            nodes.push(None);
            none_nodes.insert(2 * index + 1);
            none_nodes.insert(2 * index + 2);
        } else {
            let item = items.pop_front().unwrap();
            if item.is_none() {
                nodes.push(None);
                none_nodes.insert(2 * index + 1);
                none_nodes.insert(2 * index + 2);
            } else {
                nodes.push(Some(Rc::new(RefCell::new(TreeNode::new(item.unwrap())))));
            }
        }
        index += 1;
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

#[macro_export]
macro_rules! optional {
    (null) => {
        None
    };
    ($x:expr) => {
        Some($x)
    };
}

#[macro_export]
macro_rules! binary_tree {
    () => {
        None
    };
    ( $( $x:tt ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($crate::optional!($x));
            )*
            $crate::common::binary_tree::from_iter(temp_vec)
        }
    };
}

#[test]
fn test() {
    let tree1 = from_iter([Some(0), None, Some(2), Some(3), Some(4), Some(5), Some(6)]);
    let tree2 = binary_tree![0, null, 2, 3, 4, 5, 6];
    assert_eq!(tree1, tree2);

    let tree3 = binary_tree!(1, null, 2, 3);
    assert_eq!(tree3.as_ref().unwrap().borrow().val, 1);
    assert_eq!(tree3.as_ref().unwrap().borrow().left, None);
}
