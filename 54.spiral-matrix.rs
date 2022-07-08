/*
 * @lc app=leetcode id=54 lang=rust
 *
 * [54] Spiral Matrix
 */

// @lc code=start
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (row_count, col_count) = (matrix.len(), matrix[0].len());

        if row_count == 1 {
            return matrix[0].clone();
        }

        let mut elements = vec![0; row_count * col_count];
        let mut round = 0;
        let (mut row, mut col) = (0, 0);
        let mut index = 0;

        while index < elements.len() {
            while col < col_count - round {
                elements[index] = matrix[row][col];
                index += 1;
                if col == col_count - round - 1 {
                    break;
                }
                col += 1;
            }

            row += 1;

            while row < row_count - round {
                elements[index] = matrix[row][col];
                index += 1;
                if row == row_count - round - 1 {
                    break;
                }
                row += 1;
            }

            if col == 0 || index > elements.len() - 1 {
                break;
            }

            col -= 1;

            while col >= round {
                elements[index] = matrix[row][col];
                index += 1;
                if col == round {
                    break;
                }
                col -= 1;
            }

            if row == 0 || index > elements.len() - 1 {
                break;
            }

            row -= 1;

            while row >= round + 1 {
                elements[index] = matrix[row][col];
                index += 1;
                if row == round + 1 {
                    break;
                }
                row -= 1;
            }
            col += 1;
            round += 1;
        }

        return elements;
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let cases = [
        (
            vec![
                vec![01, 02, 03, 04],
                vec![12, 13, 14, 05],
                vec![11, 16, 15, 06],
                vec![10, 09, 08, 07],
            ],
            Vec::from_iter(1..=16),
        ),
        (vec![vec![1, 2], vec![4, 3]], vec![1, 2, 3, 4]),
        (vec![vec![1], vec![2]], vec![1, 2]),
        (vec![vec![1], vec![2], vec![3], vec![4]], vec![1, 2, 3, 4]),
        (vec![vec![1, 2]], vec![1, 2]),
        (vec![vec![1]], vec![1]),
        (vec![vec![]], vec![]),
        (vec![vec![], vec![], vec![], vec![]], vec![]),
        (
            vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]],
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
        ),
        (
            vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 9],
                vec![10, 11, 12],
            ],
            vec![1, 2, 3, 6, 9, 12, 11, 10, 7, 4, 5, 8],
        ),
    ];

    for (input, output) in cases {
        assert_eq!(Solution::spiral_order(input), output);
    }
}
