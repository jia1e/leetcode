/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        strs.sort_by(|a, b| a.len().cmp(&b.len()));
        let chars: Vec<_> = strs[0].chars().collect();
        for i in 0..chars.len() {
            for str in &strs[1..] {
                if str.chars().nth(i) != Some(chars[i]) {
                    return chars[0..i].iter().collect();
                }
            }
        }

        chars.iter().collect()
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let cases = [(["flower", "flow", "flight"], "fl")];
    for (strs, expected) in cases {
        assert_eq!(
            Solution::longest_common_prefix(strs.map(|s| s.to_string()).to_vec()),
            expected
        );
    }
}
