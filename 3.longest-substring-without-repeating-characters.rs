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
    assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
    assert_eq!(
        Solution::length_of_longest_substring("tmmzuxt".to_string()),
        5
    );

    assert_eq!(Solution::length_of_longest_substring("aab".to_string()), 2);
    assert_eq!(Solution::length_of_longest_substring("au".to_string()), 2);
    assert_eq!(Solution::length_of_longest_substring(" ".to_string()), 1);

    assert_eq!(
        Solution::length_of_longest_substring("bbbbbb".to_string()),
        1
    );

    assert_eq!(
        Solution::length_of_longest_substring("abcabcbb".to_string()),
        3
    );

    assert_eq!(
        Solution::length_of_longest_substring("pwwkew".to_string()),
        3
    );
    assert_eq!(
        Solution::length_of_longest_substring("abcdabcdef".to_string()),
        6
    );
}
