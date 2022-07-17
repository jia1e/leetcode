/*
 * @lc app=leetcode id=977 lang=rust
 *
 * [977] Squares of a Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        let (mut l, mut r) = (0, nums.len() - 1);
        while l <= r {
            if nums[r].abs() > nums[l].abs() {
                result.push(nums[r] * nums[r]);
                r -= 1;
            } else {
                result.push(nums[l] * nums[l]);
                l += 1;
            }
        }
        result.reverse();
        result
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let cases = [
        (vec![1], vec![1]),
        (vec![1, -1], vec![1, 1]),
        (vec![1, 0, -1], vec![0, 1, 1]),
        (vec![-4, -1, 0, 3, 10], vec![0, 1, 9, 16, 100]),
        (vec![-7, -3, 2, 3, 11], vec![4, 9, 9, 49, 121]),
    ];

    for (input, output) in cases {
        assert_eq!(Solution::sorted_squares(input), output);
    }
}
