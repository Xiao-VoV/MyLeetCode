/*
 * @lc app=leetcode.cn id=59 lang=javascript
 *
 * [59] 螺旋矩阵 II
 *
 * https://leetcode.cn/problems/spiral-matrix-ii/description/
 *
 * algorithms
 * Medium (70.59%)
 * Likes:    1449
 * Dislikes: 0
 * Total Accepted:    540.8K
 * Total Submissions: 764.8K
 * Testcase Example:  '3'
 *
 * 给你一个正整数 n ，生成一个包含 1 到 n^2 所有元素，且元素按顺时针顺序螺旋排列的 n x n 正方形矩阵 matrix 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：n = 3
 * 输出：[[1,2,3],[8,9,4],[7,6,5]]
 *
 *
 * 示例 2：
 *
 *
 * 输入：n = 1
 * 输出：[[1]]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 *
 *
 */

// @lc code=start
/**
 * @param {number} n
 * @return {number[][]}
 */
var generateMatrix = function (n) {
    const matrix = Array.from({ length: n }, () => Array(n).fill(0));
    let top = 0,
        bottom = n - 1;
    let left = 0,
        right = n - 1;
    let nums = 1;
    while (top <= bottom && left <= right) {
        for (let i = left; i <= right; i++) {
            matrix[top][i] = nums++;
        }
        top++;
        for (let i = top; i <= bottom; i++) {
            matrix[i][right] = nums++;
        }
        right--;
        if (top <= bottom) {
            for (let i = right; i >= left; i--) {
                matrix[bottom][i] = nums++;
            }
            bottom--;
        }

        if (left <= right) {
            for (let i = bottom; i >= top; i--) {
                matrix[i][left] = nums++;
            }
            left++;
        }
    }
    return matrix;
};
// @lc code=end
