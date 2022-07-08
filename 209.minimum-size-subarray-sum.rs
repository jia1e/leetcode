/*
 * @lc app=leetcode id=209 lang=rust
 *
 * [209] Minimum Size Subarray Sum
 */

// @lc code=start
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, 1);
        let mut min = i32::MAX;
        let mut sum = 0;
        let mut found = false;
        for index in 0..nums.len() {
            if nums[index] >= target {
                return 1;
            }

            sum += nums[index];

            if sum >= target {
                while sum >= target {
                    sum -= nums[left];
                    left += 1;
                }
                min = i32::min(min, (right - left + 1) as i32);
                found = true;
            }
            right += 1;
        }
        if found {
            min
        } else {
            0
        }
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let cases = [
        (7, vec![2, 3, 1, 2, 4, 3], 2),
        (1, vec![1, 4, 4], 1),
        (11, vec![1, 1, 1, 1, 1, 1, 1, 1], 0),
        (7, vec![7, 1, 1, 1, 1, 1, 1, 1], 1),
        (5, vec![1, 1, 1, 1, 1, 1, 1, 1], 5),
    ];

    for (target, nums, min_len) in cases {
        assert_eq!(Solution::min_sub_array_len(target, nums), min_len);
    }
}
