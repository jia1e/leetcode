/*
 * @lc app=leetcode id=347 lang=rust
 *
 * [347] Top K Frequent Elements
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counter = HashMap::<i32, i32>::new();

        for n in nums {
            let c = counter.entry(n).or_insert(0);
            *c += 1;
        }

        let mut counter: Vec<_> = counter.iter().collect();
        counter.sort_by(|(_, a), (_, b)| b.cmp(a));
        counter.truncate(k as usize);
        counter.iter().map(|(n, _)| **n).collect()
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let cases = [
        (vec![1, 1, 1, 2, 2, 3], 2, vec![1, 2]),
        (vec![1], 1, vec![1]),
    ];

    for (nums, k, output) in cases {
        assert_eq!(Solution::top_k_frequent(nums, k), output);
    }
}
