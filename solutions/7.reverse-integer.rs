/*
 * @lc app=leetcode id=7 lang=rust
 *
 * [7] Reverse Integer
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut n = x.abs();
        let mut y = 0;
        while n > 0 {
            if (i32::MAX - (n % 10)) / 10 < y {
                return 0;
            }
            y = y * 10 + n % 10;
            n /= 10;
        }
        if x.is_positive() {
            y
        } else {
            -y
        }
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);
    assert_eq!(Solution::reverse(2147483647), 0);
}
