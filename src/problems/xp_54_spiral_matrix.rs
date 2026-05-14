/*
 * @lc app=leetcode.cn id=54 lang=rust
 *
 * [54] Spiral Matrix
 *
 * https://leetcode.cn/problems/spiral-matrix/description/
 *
 * algorithms
 * Medium (55.25%)
 * Likes:    2131
 * Dislikes: 0
 * Total Accepted:    1M
 * Total Submissions: 1.8M
 * Testcase Example:  '[[1,2,3],[4,5,6],[7,8,9]]'
 *
 * Given an m x n matrix, return all elements of the matrix in spiral order.
 *
 *
 * Example 1:
 *
 *
 * Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
 * Output: [1,2,3,6,9,8,7,4,5]
 *
 *
 * Example 2:
 *
 *
 * Input: matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
 * Output: [1,2,3,4,8,12,11,10,9,5,6,7]
 *
 *
 *
 * Constraints:
 *
 *
 * m == matrix.length
 * n == matrix[i].length
 * 1 <= m, n <= 10
 * -100 <= matrix[i][j] <= 100
 *
 *
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let rows = matrix.len();
        if rows == 0 {
            return vec![];
        }
        let cols = matrix[0].len();

        let total = rows * cols;

        enum Direction {
            Right,
            Left,
            Up,
            Down,
        }

        let mut top = 0;
        let mut bottom = rows;
        let mut left = 0;
        let mut right = cols;

        let mut direction = Direction::Right;

        let mut result = vec![];
        while result.len() < total {
            match direction {
                Direction::Right => {
                    for i in left..right {
                        result.push(matrix[top][i]);
                    }
                    top += 1;
                    direction = Direction::Down;
                }
                Direction::Down => {
                    for i in top..bottom {
                        result.push(matrix[i][right - 1]);
                    }
                    right -= 1;
                    direction = Direction::Left;
                }
                Direction::Left => {
                    for i in (left..right).rev() {
                        result.push(matrix[bottom - 1][i]);
                    }
                    bottom -= 1;
                    direction = Direction::Up;
                }
                Direction::Up => {
                    for i in (top..bottom).rev() {
                        result.push(matrix[i][left]);
                    }
                    left += 1;
                    direction = Direction::Right;
                }
            }
        }

        return result;
    }
}
// @lc code=end
