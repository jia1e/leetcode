/*
 * @lc app=leetcode id=349 lang=rust
 *
 * [349] Intersection of Two Arrays
 */

// @lc code=start

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set: HashSet<i32> = HashSet::from_iter(nums1);
        let set: HashSet<i32> = nums2.into_iter().filter(|n| set.contains(n)).collect();
        set.into_iter().collect()
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let cases = [
        (vec![1, 2, 2, 1], vec![2, 2], vec![2]),
        (vec![4, 9, 5], vec![9, 4, 9, 8, 4], vec![9, 4]),
    ];

    for (nums1, nums2, expected) in cases {
        assert_eq!(Solution::intersection(nums1, nums2), expected);
    }
}
