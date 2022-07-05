/*
 * @lc app=leetcode id=35 lang=rust
 *
 * [35] Search Insert Position
 */

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len());
        while l < r {
            let m = (l + r) / 2;

            if target > nums[m] {
                l = m + 1;
            } else {
                r = m;
            }
        }

        l as i32
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::search_insert(vec![], 0), 0);
    assert_eq!(Solution::search_insert(vec![0], -1), 0);
    assert_eq!(Solution::search_insert(vec![0, 2, 4, 6], -1), 0);
    assert_eq!(Solution::search_insert(vec![0, 2, 4, 6], 0), 0);
    assert_eq!(Solution::search_insert(vec![0, 2, 4, 6], 1), 1);
    assert_eq!(Solution::search_insert(vec![0, 2, 4, 6], 2), 1);
    assert_eq!(Solution::search_insert(vec![0, 2, 4, 6], 3), 2);
    assert_eq!(Solution::search_insert(vec![0, 2, 4, 6], 5), 3);
    assert_eq!(Solution::search_insert(vec![0, 2, 4, 6], 6), 3);
    assert_eq!(Solution::search_insert(vec![0, 2, 4, 6], 7), 4);
    assert_eq!(Solution::search_insert(vec![5, 5, 5, 5], 5), 0);
    assert_eq!(Solution::search_insert(vec![4, 5, 5, 5], 5), 1);
    assert_eq!(Solution::search_insert(vec![4, 5, 5, 5], 7), 4);
}
