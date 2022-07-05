/*
 * @lc app=leetcode id=33 lang=rust
 *
 * [33] Search in Rotated Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);

        while l <= r {
            let m = (l + r) / 2;

            if nums[m] == target {
                return m as i32;
            }

            if nums[l] <= nums[m] {
                if nums[l] <= target && target < nums[m] {
                    r = m - 1
                } else {
                    l = m + 1
                }
            } else {
                if nums[m] < target && target <= nums[r] {
                    l = m + 1
                } else {
                    r = m - 1
                }
            }
        }

        if let Some(n) = nums.get(l) {
            if *n == target {
                return l as i32;
            }
        }

        -1
    }
}
// @lc code=end

pub struct Solution;

/// 边界情况的分析
/// 当 nums[l] <= nums[m] 时，只有当 m 左侧递增时才处理为 r = m - 1，否则处理为 l = m + 1;
/// 当 nums[l] > nums[m] 时同理

#[test]
fn test() {
    assert_eq!(Solution::search(vec![1, 3, 5], 2), -1);
    assert_eq!(Solution::search(vec![6, 7, 0, 1, 2, 4, 5], 2), 4);
    assert_eq!(Solution::search(vec![0, 1, 2, 4, 5, 6, 7], 0), 0);
    assert_eq!(Solution::search(vec![0, 1, 2, 4, 5, 6, 7], 1), 1);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 4), 0);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 5), 1);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 7), 3);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    assert_eq!(Solution::search(vec![7, 0, 1, 2, 4, 5, 6], 0), 1);
}
