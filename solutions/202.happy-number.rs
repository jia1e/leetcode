/*
 * @lc app=leetcode id=202 lang=rust
 *
 * [202] Happy Number
 */

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut n = n;
        let mut sum_of_squares: HashSet<i32> = HashSet::new();

        while n != 1 && !sum_of_squares.contains(&n) {
            sum_of_squares.insert(n);
            n = Self::sum_of_the_squares_of_digits(n);
        }

        n == 1
    }

    fn sum_of_the_squares_of_digits(n: i32) -> i32 {
        let mut n = n;
        let mut sum = 0;

        while n > 0 {
            sum += i32::pow(n % 10, 2);
            n /= 10;
        }

        sum
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let cases = [(19, true), (2, false)];

    for (n, expected) in cases {
        assert_eq!(Solution::is_happy(n), expected);
    }
}
