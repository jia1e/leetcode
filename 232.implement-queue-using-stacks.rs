/*
 * @lc app=leetcode id=232 lang=rust
 *
 * [232] Implement Queue using Stacks
 */

// @lc code=start
struct MyQueue<T: Default> {
    stack_in: Vec<T>,
    stack_out: Vec<T>,
}

trait Queue {
    type Item;
    fn new() -> Self;
    fn push(&mut self, x: Self::Item);
    fn pop(&mut self) -> Self::Item;
    fn peek(&mut self) -> Self::Item;
    fn empty(&self) -> bool;
}

impl<T: Default> MyQueue<T> {
    fn pop_in_push_out(&mut self) {
        if self.stack_out.is_empty() {
            while let Some(item) = self.stack_in.pop() {
                self.stack_out.push(item);
            }
        }
    }
}

impl<T: Default + Copy> Queue for MyQueue<T> {
    type Item = T;

    fn new() -> Self {
        MyQueue {
            stack_in: Vec::new(),
            stack_out: Vec::new(),
        }
    }

    fn push(&mut self, x: T) {
        self.stack_in.push(x);
    }

    fn pop(&mut self) -> T {
        self.pop_in_push_out();
        if let Some(item) = self.stack_out.pop() {
            item
        } else {
            Self::Item::default()
        }
    }

    fn peek(&mut self) -> T {
        self.pop_in_push_out();
        if !self.stack_out.is_empty() {
            self.stack_out[self.stack_out.len() - 1]
        } else {
            Self::Item::default()
        }
    }

    fn empty(&self) -> bool {
        self.stack_out.is_empty() && self.stack_in.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
// @lc code=end

#[test]
fn test() {
    let mut obj = MyQueue::new();
    obj.push(1);
    assert_eq!(obj.peek(), 1);
    obj.push(2);
    assert_eq!(obj.peek(), 1);
    obj.pop();
    assert_eq!(obj.empty(), false);
    assert_eq!(obj.peek(), 2);
    obj.pop();
    assert_eq!(obj.empty(), true);
    assert_eq!(obj.pop(), 0);
}
