/*
 * @lc app=leetcode id=26 lang=rust
 *
 * [26] Remove Duplicates from Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut k = 0;
        for i in 1..nums.len() {
            if nums[i] != nums[k] {
                k += 1;
                nums[k] = nums[i];
            }
        }
        (k + 1) as i32
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let cases = [
        (vec![], 0, vec![]),
        (vec![1], 1, vec![1]),
        (vec![1, 2], 2, vec![1, 2]),
        (vec![1, 1, 2, 2, 3, 3], 3, vec![1, 2, 3]),
    ];

    for (mut input, expected_k, output) in cases {
        let k = Solution::remove_duplicates(&mut input);
        assert_eq!(k, expected_k);
        input.truncate(k as usize);
        assert_eq!(input, output);
    }
}
