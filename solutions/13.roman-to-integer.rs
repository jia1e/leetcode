/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

// @lc code=start

use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let sv = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        let mut chars: Vec<char> = s.chars().collect();
        chars.reverse();

        let mut v = 0;

        for ch in chars {
            if sv[&ch] < v {
                result = result - sv[&ch]
            } else {
                result = result + sv[&ch]
            }

            v = sv[&ch];
        }

        result
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
}
