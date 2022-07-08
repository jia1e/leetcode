/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }

        let chars: Vec<_> = s.chars().collect();
        let mut len = 1;
        let (mut l, mut r) = (0, 1);
        let mut char_indexes = HashMap::from([(chars[l], l)]);
        while r < chars.len() {
            match char_indexes.get(&chars[r]) {
                Some(index) => {
                    if *index >= l {
                        l = index + 1;
                    } else {
                        if r - l + 1 > len {
                            len = r - l + 1;
                        }
                    }
                }
                _ => {
                    if r - l + 1 > len {
                        len = r - l + 1;
                    }
                }
            }

            char_indexes.insert(chars[r], r);
            r += 1;
        }

        len as i32
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let cases = [
        ("", 0),
        ("tmmzuxt", 5),
        ("aab", 2),
        ("au", 2),
        (" ", 1),
        ("bbbbbb", 1),
        ("abcabcbb", 3),
        ("pwwkew", 3),
        ("abcdabcdef", 6),
    ];

    for (s, len) in cases {
        assert_eq!(Solution::length_of_longest_substring(s.to_string()), len);
    }
}
