// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
pub fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Option<Box<ListNode>> {
    let mut dummy_head = ListNode::new(0);
    let mut pre = &mut dummy_head;
    for val in iter {
        pre.next = Some(Box::new(ListNode::new(val)));
        pre = pre.next.as_mut().unwrap();
    }
    dummy_head.next
}

#[allow(dead_code)]
pub fn into_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = vec![];
    let mut node = &head;
    while node.is_some() {
        result.push(node.as_ref().unwrap().val);
        node = &node.as_ref().unwrap().next
    }
    result
}

#[macro_export]
macro_rules! linked_list {
    () => {
        None
    };
    ( $( $x:expr ),* ) => {
        {
            use $crate::shared::linked_list::ListNode;
            let mut dummy_head = ListNode::new(0);
            let mut _pre = &mut dummy_head;
            $(
                _pre.next = Some(Box::new(ListNode::new($x)));
                _pre = _pre.next.as_mut().unwrap();
            )*
            dummy_head.next
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use super::from_iter;
        let list = linked_list!(1, 2, 3, 4, 5);
        assert_eq!(list, from_iter([1, 2, 3, 4, 5]));
    }
}
