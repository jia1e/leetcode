/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for ch in s.chars() {
            match ch {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                _ => {
                    if Some(ch) != stack.pop() {
                        return false;
                    }
                }
            }
        }

        stack.is_empty()
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let cases = [
        ("()[]{}", true),
        ("()", true),
        ("{}", true),
        ("[]", true),
        ("{[{()}]}", true),
        ("[{", false),
        ("{]", false),
        ("][", false),
        ("]]", false),
    ];

    for (s, expected) in cases {
        assert_eq!(Solution::is_valid(s.to_string()), expected);
    }
}
