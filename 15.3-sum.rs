/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut triplets: Vec<Vec<i32>> = vec![];
        for i in 0..(nums.len() - 2) {
            if nums[i] > 0 {
                break;
            }

            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let (mut left, mut right) = (i + 1, nums.len() - 1);

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                if sum > 0 {
                    right -= 1;
                    while right > left && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                } else if sum < 0 {
                    left += 1;
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                } else {
                    triplets.push(vec![nums[i], nums[left], nums[right]]);
                    while right > left && nums[right - 1] == nums[right] {
                        right -= 1;
                    }
                    while left < right && nums[left + 1] == nums[left] {
                        left += 1;
                    }
                    right -= 1;
                    left += 1;
                }
            }
        }

        triplets
    }

    pub fn three_sum_1(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        let nums_indexes: HashMap<&i32, usize> = nums
            .iter()
            .enumerate()
            .map(|(index, num)| (num, index))
            .collect();
        let mut triplets: Vec<Vec<i32>> = vec![];

        for i in 0..(nums.len() - 2) {
            if nums[i] > 0 {
                break;
            }
            for j in (i + 1)..(nums.len() - 1) {
                let third_num = -nums[i] - nums[j];
                if third_num < 0 {
                    break;
                }

                let found = triplets
                    .iter()
                    .find(|t| t[0] == nums[i] && t[1] == nums[j] && t[2] == third_num);

                if found.is_none() {
                    if let Some(k) = nums_indexes.get(&third_num) {
                        if *k > j {
                            triplets.push(vec![nums[i], nums[j], third_num]);
                        }
                    }
                }
            }
        }

        triplets
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let cases = [
        (vec![-1, 0, 1, 2, -1, -4], vec![[-1, -1, 2], [-1, 0, 1]]),
        (vec![0, 1, 1], vec![]),
        (
            vec![-4, -2, 1, -5, -4, -4, 4, -2, 0, 4, 0, -2, 3, 1, -5, 0],
            vec![
                [-5, 1, 4],
                [-4, 0, 4],
                [-4, 1, 3],
                [-2, -2, 4],
                [-2, 1, 1],
                [0, 0, 0],
            ],
        ),
    ];

    for (nums, expected) in cases {
        let mut output: Vec<_> = Solution::three_sum(nums)
            .into_iter()
            .map(|mut triplets| {
                triplets.sort();
                triplets
            })
            .collect();
        output.sort_by(|a, b| {
            for (n, m) in a.iter().zip(b) {
                if n != m {
                    return a.cmp(b);
                }
            }
            std::cmp::Ordering::Equal
        });
        assert_eq!(output, expected);
    }
}
