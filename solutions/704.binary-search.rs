/*
 * @lc app=leetcode id=704 lang=rust
 *
 * [704] Binary Search
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len());

        while l < r {
            let m = (l + r) / 2;

            if target > nums[m] {
                l = m + 1;
            } else if target < nums[m] {
                r = m;
            } else {
                return m as i32;
            }
        }

        -1
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::search(vec![1], 1), 0);
    assert_eq!(Solution::search(vec![1, 2], 1), 0);
    assert_eq!(Solution::search(vec![1, 2, 3], 1), 0);
    assert_eq!(Solution::search(vec![1, 2, 3, 4], 1), 0);
    assert_eq!(Solution::search(vec![1, 2, 3, 4, 5, 6], 3), 2);
    assert_eq!(Solution::search(vec![1, 2, 3, 4, 5, 6], 6), 5);
    assert_eq!(Solution::search(vec![1], 3), -1);
    assert_eq!(Solution::search(vec![1, 2, 3, 4, 5, 6], 0), -1);
    assert_eq!(Solution::search(vec![1, 2, 3, 4, 5, 7], 8), -1);
}

impl Solution {
    pub fn search_2(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);

        while l <= r {
            let m = (l + r) / 2;
            if nums[m] > target {
                // 处理 usize 溢出
                if m == 0 {
                    return -1;
                }
                r = m - 1;
            } else if nums[m] < target {
                l = m + 1;
            } else {
                return m as i32;
            }
        }

        -1
    }
}

#[test]
fn test_2() {
    assert_eq!(Solution::search_2(vec![1], 1), 0);
    assert_eq!(Solution::search_2(vec![1, 2], 1), 0);
    assert_eq!(Solution::search_2(vec![1, 2, 3], 1), 0);
    assert_eq!(Solution::search_2(vec![1, 2, 3, 4], 1), 0);
    assert_eq!(Solution::search_2(vec![1, 2, 3, 4, 5, 6], 3), 2);
    assert_eq!(Solution::search_2(vec![1, 2, 3, 4, 5, 6], 6), 5);
    assert_eq!(Solution::search_2(vec![1], 3), -1);
    assert_eq!(Solution::search_2(vec![1, 2, 3, 4, 5, 6], 0), -1);
    assert_eq!(Solution::search_2(vec![1, 2, 3, 4, 5, 7], 8), -1);
}
