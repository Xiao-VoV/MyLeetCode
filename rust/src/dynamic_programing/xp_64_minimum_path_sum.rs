use std::{mem, os::unix::raw::gid_t};

/*
 * @lc app=leetcode.cn id=64 lang=rust
 *
 * [64] Minimum Path Sum
 *
 * https://leetcode.cn/problems/minimum-path-sum/description/
 *
 * algorithms
 * Medium (72.50%)
 * Likes:    1967
 * Dislikes: 0
 * Total Accepted:    1M
 * Total Submissions: 1.4M
 * Testcase Example:  '[[1,3,1],[1,5,1],[4,2,1]]'
 *
 * Given a m x n grid filled with non-negative numbers, find a path from top
 * left to bottom right, which minimizes the sum of all numbers along its
 * path.
 *
 * Note: You can only move either down or right at any point in time.
 *
 *
 * Example 1:
 *
 *
 * Input: grid = [[1,3,1],[1,5,1],[4,2,1]]
 * Output: 7
 * Explanation: Because the path 1 → 3 → 1 → 1 → 1 minimizes the sum.
 *
 *
 * Example 2:
 *
 *
 * Input: grid = [[1,2,3],[4,5,6]]
 * Output: 12
 *
 *
 *
 * Constraints:
 *
 *
 * m == grid.length
 * n == grid[i].length
 * 1 <= m, n <= 200
 * 0 <= grid[i][j] <= 200
 *
 *
 */
use crate::Solution;
// @lc code=start
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut mem: Vec<Vec<Option<i32>>> = vec![vec![None; n]; m];

        Self::dp_64(m - 1, n - 1, &mut mem, &grid)
    }

    fn dp_64(m: usize, n: usize, mem: &mut Vec<Vec<Option<i32>>>, grid: &Vec<Vec<i32>>) -> i32 {
        if m == 0 && n == 0 {
            return grid[0][0];
        }

        if let Some(result) = mem[m][n] {
            return result;
        }

        let sum = if m == 0 {
            Self::dp_64(m, n - 1, mem, grid) + grid[m][n]
        } else if n == 0 {
            Self::dp_64(m - 1, n, mem, grid) + grid[m][n]
        } else {
            Self::dp_64(m - 1, n, mem, grid).min(Self::dp_64(m, n - 1, mem, grid)) + grid[m][n]
        };

        mem[m][n] = Some(sum);

        sum
    }
}
// @lc code=end
