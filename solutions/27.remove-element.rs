/*
 * @lc app=leetcode id=27 lang=rust
 *
 * [27] Remove Element
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k += 1;
            }
        }

        k as i32
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let k = Solution::remove_element(&mut nums, 2);
    nums.truncate(k as usize);
    assert_eq!(nums, vec![0, 1, 3, 0, 4]);

    let mut nums = vec![];
    let k = Solution::remove_element(&mut nums, 1);
    nums.truncate(k as usize);
    assert_eq!(nums, vec![]);

    let mut nums = vec![1];
    let k = Solution::remove_element(&mut nums, 1);
    nums.truncate(k as usize);
    assert_eq!(nums, vec![]);

    let mut nums = vec![1, 1, 1, 1, 2];
    let k = Solution::remove_element(&mut nums, 1);
    nums.truncate(k as usize);
    assert_eq!(nums, vec![2]);

    let mut nums = vec![1, 1, 3, 1, 2];
    let k = Solution::remove_element(&mut nums, 1);
    nums.truncate(k as usize);
    assert_eq!(nums, vec![3, 2]);

    let mut nums = vec![1, 1, 3, 1, 2];
    let k = Solution::remove_element(&mut nums, 3);
    nums.truncate(k as usize);
    assert_eq!(nums, vec![1, 1, 1, 2]);

    let mut nums = vec![1, 1, 3, 1, 2];
    let k = Solution::remove_element(&mut nums, 4);
    nums.truncate(k as usize);
    assert_eq!(nums, vec![1, 1, 3, 1, 2]);
}
