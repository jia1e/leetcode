/*
 * @lc app=leetcode id=28 lang=rust
 *
 * [28] Implement strStr()
 */

// @lc code=start
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() == 0 {
            return 0;
        }
        let mut index = 0;
        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();
        'outer: while index + needle.len() <= haystack.len() {
            for (a, b) in needle.iter().zip(&haystack[index..index + needle.len()]) {
                if a != b {
                    index += 1;
                    continue 'outer;
                }
            }
            return index as i32;
        }
        -1
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let cases = [
        ("a", "a", 0),
        ("abc", "c", 2),
        ("hello", "ll", 2),
        ("", "", 0),
        ("a", "ab", -1),
        ("aaaaa", "bba", -1),
    ];

    for (haystack, needle, expected) in cases {
        assert_eq!(
            Solution::str_str(haystack.to_string(), needle.to_string()),
            expected
        );
    }
}
