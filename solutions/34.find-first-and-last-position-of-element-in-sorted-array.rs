/*
 * @lc app=leetcode id=34 lang=rust
 *
 * [34] Find First and Last Position of Element in Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut l, mut r) = (0, nums.len());
        let mut o = -1;

        while l < r {
            let m = (l + r) / 2;

            if nums[m] > target {
                r = m;
            } else if nums[m] < target {
                l = m + 1;
            } else {
                o = m as i32;
                break;
            }
        }

        if o == -1 {
            vec![o, o]
        } else {
            let (mut a, mut b) = (o as usize, o as usize);

            while l < a {
                let m = (l + a) / 2;
                if nums[m] < target {
                    l = m + 1;
                } else {
                    a = m
                }
            }

            while r > b {
                let m = (b + r) / 2;
                if b == m {
                    if r < nums.len() && nums[r] == target {
                        b += 1;
                    }
                    break;
                }
                if nums[m] > target {
                    r = m - 1
                } else {
                    b = m
                }
            }

            vec![a as i32, b as i32]
        }
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::search_range(vec![1, 2, 3, 4, 5], 1), [0, 0]);
    assert_eq!(Solution::search_range(vec![1, 2, 3, 4, 5], 2), [1, 1]);
    assert_eq!(Solution::search_range(vec![1, 2, 3, 4, 5], 3), [2, 2]);
    assert_eq!(Solution::search_range(vec![1, 2, 3, 4, 5], 4), [3, 3]);
    assert_eq!(Solution::search_range(vec![1, 2, 3, 4, 5], 5), [4, 4]);
    assert_eq!(Solution::search_range(vec![1, 1, 1, 4, 5], 1), [0, 2]);
    assert_eq!(Solution::search_range(vec![1, 2, 5, 5, 5], 5), [2, 4]);
    assert_eq!(Solution::search_range(vec![5, 5, 5, 5, 5], 5), [0, 4]);
    assert_eq!(Solution::search_range(vec![1, 2, 3, 4, 5], 0), [-1, -1]);
    assert_eq!(Solution::search_range(vec![1, 2, 3, 4, 5], 6), [-1, -1]);
}
