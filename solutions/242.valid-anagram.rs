/*
 * @lc app=leetcode id=242 lang=rust
 *
 * [242] Valid Anagram
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut counters = [0; 26];
        let base = b'a';

        for byte in s.bytes() {
            counters[(byte - base) as usize] += 1;
        }

        for byte in t.bytes() {
            if counters[(byte - base) as usize] == 0 {
                return false;
            }

            counters[(byte - base) as usize] -= 1;
        }

        counters.iter().all(|a| *a == 0)
    }

    pub fn is_anagram_2(s: String, t: String) -> bool {
        let mut letters = HashMap::new();

        for ch in s.chars() {
            let counter = letters.entry(ch).or_insert(0);
            *counter += 1;
        }

        for ch in t.chars() {
            let counter = letters.entry(ch).or_insert(-1);
            if *counter == -1 {
                return false;
            }

            *counter -= 1;
        }

        letters.iter().all(|(_, counter)| *counter == 0)
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let cases = [
        ("anagram", "nagaram", true),
        ("rat", "tar", true),
        ("rat", "car", false),
    ];

    for (s, t, expected) in cases {
        assert_eq!(Solution::is_anagram(s.to_string(), t.to_string()), expected);
    }
}
