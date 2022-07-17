/// 思路
/// 使用左移右移实现乘2除2

/*
 * @lc app=leetcode id=29 lang=rust
 *
 * [29] Divide Two Integers
 */

// @lc code=start
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if divisor == 1 {
            dividend
        } else if divisor == -1 {
            if dividend == i32::MIN {
                i32::MAX
            } else {
                -dividend
            }
        } else {
            let (dividend, divisor) = (dividend as i64, divisor as i64);
            let mut is_negative = false;
            let dividend_abs = if dividend < 0 {
                is_negative = !is_negative;
                -dividend
            } else {
                dividend
            };
            let divisor_abs = if divisor < 0 {
                is_negative = !is_negative;
                -divisor
            } else {
                divisor
            };

            if dividend_abs < divisor_abs {
                0
            } else {
                let mut multiples = vec![(1, divisor_abs)];
                let mut index = 0_i32;
                let mut result = 1;
                let mut sum = divisor_abs;

                loop {
                    let temp = sum + sum;
                    if temp > dividend_abs {
                        break;
                    }

                    sum = temp;
                    index += 1;
                    result += result;
                    multiples.push((result, sum));
                }

                while index >= 0 {
                    let temp = multiples[index as usize].1 + sum;
                    if temp <= dividend_abs {
                        sum = temp;
                        result += multiples[index as usize].0;
                    }
                    index -= 1;
                }

                if is_negative {
                    -result
                } else {
                    result
                }
            }
        }
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    println!("{}", i32::MIN);
    assert_eq!(Solution::divide(10, 1), 10);
    assert_eq!(Solution::divide(10, 2), 5);
    assert_eq!(Solution::divide(10, 3), 3);
    assert_eq!(Solution::divide(10, 4), 2);
    assert_eq!(Solution::divide(10, 5), 2);
    assert_eq!(Solution::divide(10, 6), 1);
    assert_eq!(Solution::divide(-2147483648, -1), 2147483647);
    assert_eq!(Solution::divide(2147483647, 2), 1073741823);
}
