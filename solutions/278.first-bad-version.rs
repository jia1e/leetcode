/*
 * @lc app=leetcode id=278 lang=rust
 *
 * [278] First Bad Version
 */

// @lc code=start
// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut l, mut r) = (0, n);
        while l <= r {
            // 等同 let m = (l + r) / 2，防止 l + r 溢出
            let m = l + (r - l) / 2;
            if self.isBadVersion(m) {
                r = m - 1
            } else {
                l = m + 1
            }
        }

        l
    }
}
// @lc code=end

pub struct Solution(i32);

impl Solution {
    #[allow(non_snake_case)]
    fn isBadVersion(&self, version: i32) -> bool {
        version >= self.0
    }
}

#[test]
fn test() {
    let cases = [
        (1, 1),
        (2, 2),
        (1, i32::MAX),
        (100, i32::MAX),
        (1702766719, 2126753390),
    ];
    for (except, input) in cases {
        println!("{except} {input}");
        assert_eq!(Solution(except).first_bad_version(input), except);
    }
}
