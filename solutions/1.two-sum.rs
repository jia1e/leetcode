/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_index = HashMap::new();
        for (i, &n) in nums.iter().enumerate() {
            match nums_index.get(&(target - n)) {
                Some(&j) => return vec![j as i32, i as i32],
                None => {
                    nums_index.insert(n, i);
                }
            }
        }
        vec![-1, -1]
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let cases = [
        (vec![1, 2, 3, 4, 5], 6, vec![1,3]),
        (vec![3, 2, 4], 6, vec![1, 2]),
    ];

    for (nums, target, expected) in cases {
        assert_eq!(Solution::two_sum(nums, target), expected);
    }
}
