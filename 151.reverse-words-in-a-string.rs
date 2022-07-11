/*
 * @lc app=leetcode id=151 lang=rust
 *
 * [151] Reverse Words in a String
 */

// @lc code=start
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut chars: Vec<_> = s.chars().collect();
        let mut last_word_begin = 0;
        let mut index = 0;
        let mut cursor = 0;
        while index < chars.len() {
            if chars[index] == ' ' {
                if cursor == 0 {
                    index += 1;
                    continue;
                }

                chars[cursor] = ' ';
                cursor += 1;
                index += 1;
                last_word_begin = cursor;

                while index < chars.len() && chars[index] == ' ' {
                    index += 1;
                }
            } else {
                chars[cursor] = chars[index];
                if index == chars.len() - 1 || chars[index + 1] == ' ' {
                    let (mut left, mut right) = (last_word_begin, cursor);
                    while left < right {
                        chars.swap(left, right);
                        left += 1;
                        right -= 1;
                    }
                }
                cursor += 1;
                index += 1;
            }
        }
        chars.truncate(if chars[cursor - 1] == ' ' {
            cursor - 1
        } else {
            cursor
        });
        chars.reverse();
        chars.iter().collect()
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let cases = [(" the  sky  is  blue ", "blue is sky the")];
    for (input, output) in cases {
        assert_eq!(
            Solution::reverse_words(input.to_string()),
            output.to_string()
        );
    }
}
