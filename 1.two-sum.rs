/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let nums_index: HashMap<&i32, usize> = nums
            .iter()
            .enumerate()
            .map(|(index, num)| (num, index))
            .collect();

        for (i, n) in nums.iter().enumerate() {
            if let Some(result) = nums_index.get(&(target - n)).and_then(|index| {
                if index != &i {
                    Some(vec![i as i32, *index as i32])
                } else {
                    None
                }
            }) {
                return result;
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
        (vec![1, 2, 3, 4, 5], 6, vec![0, 4]),
        (vec![3, 2, 4], 6, vec![1, 2]),
    ];

    for (nums, target, expected) in cases {
        assert_eq!(Solution::two_sum(nums, target), expected);
    }
}
