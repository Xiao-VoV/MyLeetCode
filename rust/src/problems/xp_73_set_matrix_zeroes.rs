/*
 * @lc app=leetcode.cn id=73 lang=rust
 *
 * [73] Set Matrix Zeroes
 *
 * https://leetcode.cn/problems/set-matrix-zeroes/description/
 *
 * algorithms
 * Medium (71.60%)
 * Likes:    1371
 * Dislikes: 0
 * Total Accepted:    795.9K
 * Total Submissions: 1.1M
 * Testcase Example:  '[[1,1,1],[1,0,1],[1,1,1]]'
 *
 * Given an m x n integer matrix matrix, if an element is 0, set its entire row
 * and column to 0's.
 *
 * You must do it in place.
 *
 *
 * Example 1:
 *
 *
 * Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
 * Output: [[1,0,1],[0,0,0],[1,0,1]]
 *
 *
 * Example 2:
 *
 *
 * Input: matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
 * Output: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]
 *
 *
 *
 * Constraints:
 *
 *
 * m == matrix.length
 * n == matrix[0].length
 * 1 <= m, n <= 200
 * -2^31 <= matrix[i][j] <= 2^31 - 1
 *
 *
 *
 * Follow up:
 *
 *
 * A straightforward solution using O(mn) space is probably a bad idea.
 * A simple improvement uses O(m + n) space, but still not the best
 * solution.
 * Could you devise a constant space solution?
 *
 *
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let check = matrix.clone();

        let col = matrix.len();
        if col == 0 {
            return;
        }
        let row = matrix[0].len();

        for i in 0..row {
            for j in 0..col {
                if matrix[i][j] == 0 {
                    for c in 0..col {
                        matrix[i][c] == 0;
                    }
                    for r in 0..row {
                        matrix[r][j] == 0;
                    }
                }
            }
        }
    }
}
// @lc code=end
