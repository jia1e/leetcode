/*
 * @lc app=leetcode id=59 lang=rust
 *
 * [59] Spiral Matrix II
 */

// @lc code=start
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        if n == 1 {
            return vec![vec![1]];
        }

        let n = n as usize;
        let round = n / 2 + n % 2;
        let mut matrix = vec![vec![0; n]; n];
        let mut current_value = 1;

        let (mut row, mut col) = (0, 0);
        for r in 0..round {
            while col < n - r {
                matrix[row][col] = current_value;
                current_value += 1;
                if col == n - r - 1 {
                    break;
                }
                col += 1;
            }

            row += 1;

            while row < n - r {
                matrix[row][col] = current_value;
                current_value += 1;
                if row == n - r - 1 {
                    break;
                }
                row += 1;
            }

            col -= 1;

            while col >= r {
                matrix[row][col] = current_value;
                current_value += 1;
                if col == r {
                    break;
                }
                col -= 1;
            }

            row -= 1;

            while row > r {
                matrix[row][col] = current_value;
                current_value += 1;
                if row == r + 1 {
                    break;
                }
                row -= 1;
            }

            col += 1;
        }

        matrix
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let cases = [
        (1, vec![vec![1]]),
        (2, vec![vec![1, 2], vec![4, 3]]),
        (3, vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]),
        (
            4,
            vec![
                vec![01, 02, 03, 04],
                vec![12, 13, 14, 05],
                vec![11, 16, 15, 06],
                vec![10, 09, 08, 07],
            ],
        ),
        (
            5,
            vec![
                vec![01, 02, 03, 04, 05],
                vec![16, 17, 18, 19, 06],
                vec![15, 24, 25, 20, 07],
                vec![14, 23, 22, 21, 08],
                vec![13, 12, 11, 10, 09],
            ],
        ),
    ];

    for (n, matrix) in cases {
        assert_eq!(Solution::generate_matrix(n), *matrix);
    }
}
