/*
 * @lc app=leetcode id=541 lang=rust
 *
 * [541] Reverse String II
 */

// @lc code=start
impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let k = k as usize;
        let mut chars: Vec<_> = s.chars().collect();

        let mut start = 0;

        while start < s.len() {
            let mut left = start;
            let mut right = usize::min(start + k - 1, s.len() - 1);

            while left < right {
                chars.swap(left, right);
                left += 1;
                right -= 1;
            }

            start += 2 * k;
        }

        chars.into_iter().collect()
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let cases = [("abcdefg", 2, "bacdfeg"), ("abcd", 2, "bacd")];

    for (s, k, expected) in cases {
        assert_eq!(Solution::reverse_str(s.to_string(), k), expected);
    }
}
