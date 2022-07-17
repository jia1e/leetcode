/*
 * @lc app=leetcode id=283 lang=rust
 *
 * [283] Move Zeroes
 */

// @lc code=start
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut non_zero_count = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(non_zero_count, i);
                non_zero_count += 1;
            }
        }
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let cases = [
        (vec![0], vec![0]),
        (vec![1], vec![1]),
        (vec![1, 2, 3], vec![1, 2, 3]),
        (vec![1, 2, 3, 0], vec![1, 2, 3, 0]),
        (vec![0, 1, 0, 2, 0, 3, 0], vec![1, 2, 3, 0, 0, 0, 0]),
        (vec![0, 1, 0, 3, 12], vec![1, 3, 12, 0, 0]),
    ];

    for (mut input, expected) in cases {
        Solution::move_zeroes(&mut input);
        assert_eq!(input, expected);
    }
}
