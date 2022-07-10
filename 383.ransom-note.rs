/*
 * @lc app=leetcode id=383 lang=rust
 *
 * [383] Ransom Note
 */

// @lc code=start
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut char_counters = [0; 26];
        let base = b'a';
        for byte in magazine.bytes() {
            char_counters[(byte - base) as usize] += 1;
        }

        for byte in ransom_note.bytes() {
            char_counters[(byte - base) as usize] -= 1;
            if char_counters[(byte - base) as usize] < 0 {
                return false;
            }
        }

        return true;
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let cases = [
        ("a", "b", false),
        ("aa", "ab", false),
        ("aa", "aab", true),
        ("", "", true),
    ];

    for (ransom_note, magazine, expected) in cases {
        assert_eq!(
            Solution::can_construct(ransom_note.to_string(), magazine.to_string()),
            expected
        );
    }
}
