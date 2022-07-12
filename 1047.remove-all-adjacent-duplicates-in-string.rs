/*
 * @lc app=leetcode id=1047 lang=rust
 *
 * [1047] Remove All Adjacent Duplicates In String
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        if s.len() < 2 {
            return s;
        }

        let mut chars: Vec<_> = s.chars().collect();

        let (mut bottom, mut top) = (0, 1);
        let mut i = 1;

        while i < chars.len() {
            if top > bottom {
                if chars[i] == chars[top - 1] {
                    top -= 1;
                    i += 1;
                } else {
                    chars[top] = chars[i];
                    top += 1;
                    i += 1;
                }
            } else {
                if chars[i] == chars[i + 1] {
                    bottom = i + 2;
                    top = i + 3;
                    i += 3;
                } else {
                    chars[top] = chars[i];
                    top += 1;
                    i += 1;
                }
            }
        }

        if bottom < chars.len() {
            chars[bottom..top].iter().collect()
        } else {
            String::new()
        }
    }
}
// @lc code=end

pub struct Solution;

impl Solution {
    pub fn remove_duplicates_2(s: String) -> String {
        let mut chars: Vec<_> = s.chars().collect();
        let mut len = 0;

        for i in 0..chars.len() {
            if len > 0 {
                if chars[len - 1] == chars[i] {
                    len -= 1;
                } else {
                    chars[len] = chars[i];
                    len += 1;
                }
            } else {
                chars[len] = chars[i];
                len += 1;
            }
        }

        chars[0..len].iter().collect()
    }

    pub fn remove_duplicates_3(s: String) -> String {
        let mut chars = vec![];
        for ch in s.chars() {
            if Some(&ch) == chars.last() {
                chars.pop();
            } else {
                chars.push(ch);
            }
        }
        chars.iter().collect()
    }
}

#[test]
fn test() {
    let cases = [
        ("", ""),
        ("abeccdde", "ab"),
        ("aa", ""),
        ("abba", ""),
        ("cabbad", "cd"),
        ("abaabacd", "cd"),
        ("aaaaaaaaa", "a"),
        ("aaabbacd", "cd"),
        ("aaabbacd", "cd"),
        ("aaaaaaaa", ""),
    ];

    for (s, expected) in cases {
        assert_eq!(
            Solution::remove_duplicates(s.to_string()),
            expected.to_string()
        );
        assert_eq!(
            Solution::remove_duplicates_2(s.to_string()),
            expected.to_string()
        );
        assert_eq!(
            Solution::remove_duplicates_3(s.to_string()),
            expected.to_string()
        );
    }
}
