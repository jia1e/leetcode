/*
 * @lc app=leetcode id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 */

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (l1, l2) = (nums1.len(), nums2.len());
        let l = l1 + l2;
        let is_odd = l % 2 == 1;
        let m = (l1 + l2) / 2 + 1;
        let mut i = 0;
        let (mut i1, mut i2) = (0, 0);
        let (mut last_value, mut current_value) = (0, 0);

        while (i1 < l1 || i2 < l2) && i < m {
            match (nums1.get(i1), nums2.get(i2)) {
                (Some(n1), Some(n2)) => {
                    last_value = current_value;
                    if n1 <= n2 {
                        i1 += 1;
                        current_value = *n1;
                    } else {
                        i2 += 1;
                        current_value = *n2;
                    }
                    i += 1;
                }
                (Some(n1), None) => {
                    i1 += 1;
                    i += 1;
                    last_value = current_value;
                    current_value = *n1;
                }
                (None, Some(n2)) => {
                    i2 += 1;
                    i += 1;
                    last_value = current_value;
                    current_value = *n2;
                }
                (None, None) => {}
            }
        }

        if is_odd {
            current_value.into()
        } else {
            (last_value as f64 + current_value as f64) / 2.0
        }
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![]), 0.0);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![]), 1.0);
    assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![2]), 1.5);

    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8]),
        5.0
    );

    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 3, 5, 7], vec![2, 4, 6, 8]),
        4.5
    );

    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1], vec![2, 4, 6, 8]),
        4.0
    );
}
