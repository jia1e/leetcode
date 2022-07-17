/*
 * @lc app=leetcode id=844 lang=rust
 *
 * [844] Backspace String Compare
 */

// @lc code=start
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        Self::parse_string(s) == Self::parse_string(t)
    }

    fn parse_string(input: String) -> Vec<char> {
        let mut output: Vec<char> = vec![];
        for ch in input.chars() {
            match ch {
                '#' => {
                    output.pop();
                }
                _ => output.push(ch),
            };
        }
        output
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let cases = [
        ("ab#c", "ad#c", true),
        ("", "", true),
        ("ab##", "c#d#", true),
        ("a#c", "b", false),
    ];

    for (s, t, output) in cases {
        assert_eq!(
            Solution::backspace_compare(s.to_string(), t.to_string()),
            output
        );
    }
}
