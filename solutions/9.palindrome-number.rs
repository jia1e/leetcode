/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        if x < 10 {
            return true;
        }

        if x % 10 == 0 {
            return false;
        }

        let mut x = x;
        let mut n = 0;

        while n < x {
            n = n * 10 + x % 10;
            x = x / 10
        }

        x == n || x == n / 10
    }
}
// @lc code=end

pub struct Solution;
