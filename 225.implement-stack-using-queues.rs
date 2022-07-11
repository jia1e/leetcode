/*
 * @lc app=leetcode id=225 lang=rust
 *
 * [225] Implement Stack using Queues
 */

// @lc code=start
use std::collections::VecDeque;
pub struct MyStack {
    primary: VecDeque<i32>,
    secondary: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    pub fn new() -> Self {
        Self {
            primary: VecDeque::new(),
            secondary: VecDeque::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        while !self.primary.is_empty() {
            self.secondary.push_back(self.primary.pop_front().unwrap());
        }

        self.primary.push_back(x);

        while !self.secondary.is_empty() {
            self.primary.push_back(self.secondary.pop_front().unwrap());
        }
    }

    pub fn pop(&mut self) -> i32 {
        if self.primary.is_empty() {
            0
        } else {
            self.primary.pop_front().unwrap()
        }
    }

    pub fn top(&self) -> i32 {
        if let Some(top) = self.primary.front() {
            *top
        } else {
            0
        }
    }

    pub fn empty(&self) -> bool {
        self.primary.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */
// @lc code=end

#[test]
fn test() {
    let mut obj = MyStack::new();
    assert_eq!(obj.empty(), true);
    obj.push(1);
    assert_eq!(obj.empty(), false);
    obj.push(2);
    assert_eq!(obj.top(), 2);
    assert_eq!(obj.pop(), 2);
    assert_eq!(obj.top(), 1);
    obj.push(3);
    obj.push(4);
    obj.push(5);
    assert_eq!(obj.top(), 5);
}
