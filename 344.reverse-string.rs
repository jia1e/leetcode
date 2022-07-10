/*
 * @lc app=leetcode id=344 lang=rust
 *
 * [344] Reverse String
 */

// @lc code=start
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let (mut left, mut right) = (0, s.len() - 1);
        while left < right {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let cases = [
        vec!['h', 'e', 'l', 'l', 'o'],
        vec!['H', 'a', 'n', 'n', 'a', 'h'],
    ];

    for mut s in cases {
        let mut expected = s.clone();
        expected.reverse();
        Solution::reverse_string(&mut s);
        assert_eq!(s, expected);
    }
}
