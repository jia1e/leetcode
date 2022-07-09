/*
 * @lc app=leetcode id=707 lang=rust
 *
 * [707] Design Linked List
 */

// @lc code=start
pub struct MyLinkedList {
    head: Option<Box<LinkedListNode>>,
}

pub struct LinkedListNode {
    val: i32,
    next: Option<Box<LinkedListNode>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn get(&self, index: i32) -> i32 {
        let mut current = &self.head;
        for _ in 0..index {
            if let Some(node) = current {
                current = &node.next
            } else {
                return -1;
            }
        }

        if let Some(node) = current.as_ref() {
            return node.val;
        }

        -1
    }

    pub fn add_at_head(&mut self, val: i32) {
        self.head = Some(Box::new(LinkedListNode {
            val,
            next: self.head.take(),
        }))
    }

    pub fn add_at_tail(&mut self, val: i32) {
        let mut tail = &mut self.head;
        while tail.is_some() {
            tail = &mut tail.as_mut().unwrap().next
        }
        tail.replace(Box::new(LinkedListNode { val, next: None }));
    }

    pub fn add_at_index(&mut self, index: i32, val: i32) {
        if index == 0 {
            self.add_at_head(val);
            return;
        }

        let mut pre = &mut self.head;
        for _ in 0..index - 1 {
            if let Some(node) = pre {
                pre = &mut node.next
            } else {
                return;
            }
        }

        if let Some(node) = pre {
            let next = node.next.take();
            node.next = Some(Box::new(LinkedListNode { val, next }));
        }
    }

    pub fn delete_at_index(&mut self, index: i32) {
        if index == 0 {
            if self.head.is_some() {
                self.head = self.head.as_mut().unwrap().next.take();
            }
            return;
        }

        let mut pre = &mut self.head;
        for _ in 0..index - 1 {
            if let Some(node) = pre {
                pre = &mut node.next;
            } else {
                return;
            }
        }

        if let Some(node) = pre {
            let next = node.next.take();
            if let Some(node) = next {
                pre.as_mut().unwrap().next = node.next;
            }
        }
    }
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */
// @lc code=end

#[test]
fn test() {
    let mut list = MyLinkedList::new();
    assert_eq!(list.get(0), -1);
    list.add_at_head(0);
    list.add_at_tail(1);
    list.add_at_tail(2);
    assert_eq!(list.get(0), 0);
    assert_eq!(list.get(1), 1);
    assert_eq!(list.get(2), 2);
    list.add_at_head(3);
    list.add_at_head(4);
    assert_eq!(list.get(0), 4);
    assert_eq!(list.get(1), 3);
    assert_eq!(list.get(2), 0);
    assert_eq!(list.get(3), 1);
    assert_eq!(list.get(4), 2);
    list.add_at_index(3, 5);
    assert_eq!(list.get(3), 5);
    assert_eq!(list.get(4), 1);
    list.delete_at_index(3);
    assert_eq!(list.get(3), 1);
    list.delete_at_index(0);
    assert_eq!(list.get(0), 3);
    list.add_at_index(0, 6);
    assert_eq!(list.get(0), 6);
    assert_eq!(list.get(1), 3);

    let mut list = MyLinkedList::new();
    list.add_at_head(1);
    list.add_at_tail(3);
    list.add_at_index(1, 2);
    list.get(1);
    list.delete_at_index(1);
    list.get(1);
    list.get(3);
    list.delete_at_index(3);
    list.delete_at_index(0);
    list.get(0);
    list.delete_at_index(0);
    list.get(0);

    let mut list = MyLinkedList::new();
    list.add_at_head(1);
    list.add_at_tail(3);
    list.add_at_index(3, 2);
}
