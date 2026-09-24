/*
 * @lc app=leetcode.cn id=63 lang=rust
 *
 * [63] Unique Paths II
 *
 * https://leetcode.cn/problems/unique-paths-ii/description/
 *
 * algorithms
 * Medium (42.66%)
 * Likes:    1518
 * Dislikes: 0
 * Total Accepted:    729.7K
 * Total Submissions: 1.7M
 * Testcase Example:  '[[0,0,0],[0,1,0],[0,0,0]]'
 *
 * You are given an m x n integer array grid. There is a robot initially
 * located at the top-left corner (i.e., grid[0][0]). The robot tries to move
 * to the bottom-right corner (i.e., grid[m - 1][n - 1]). The robot can only
 * move either down or right at any point in time.
 *
 * An obstacle and space are marked as 1 or 0 respectively in grid. A path that
 * the robot takes cannot include any square that is an obstacle.
 *
 * Return the number of possible unique paths that the robot can take to reach
 * the bottom-right corner.
 *
 * The testcases are generated so that the answer will be less than or equal to
 * 2 * 10^9.
 *
 *
 * Example 1:
 *
 *
 * Input: obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
 * Output: 2
 * Explanation: There is one obstacle in the middle of the 3x3 grid above.
 * There are two ways to reach the bottom-right corner:
 * 1. Right -> Right -> Down -> Down
 * 2. Down -> Down -> Right -> Right
 *
 *
 * Example 2:
 *
 *
 * Input: obstacleGrid = [[0,1],[0,0]]
 * Output: 1
 *
 *
 *
 * Constraints:
 *
 *
 * m == obstacleGrid.length
 * n == obstacleGrid[i].length
 * 1 <= m, n <= 100
 * obstacleGrid[i][j] is 0 or 1.
 *
 *
 */

// @lc code=start
// @lc code=start
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();

        // 起点或终点是障碍，直接返回 0
        if obstacle_grid[0][0] == 1 || obstacle_grid[m - 1][n - 1] == 1 {
            return 0;
        }

        let mut memo = vec![vec![None; n]; m];
        Self::dp(m - 1, n - 1, &obstacle_grid, &mut memo)
    }

    fn dp(m: usize, n: usize, grid: &Vec<Vec<i32>>, memo: &mut Vec<Vec<Option<i32>>>) -> i32 {
        if m == 0 && n == 0 {
            return 1; // 起点，一定可达（前面已经检查过不是障碍）
        }
        if grid[m][n] == 1 {
            return 0; // 障碍物
        }
        if let Some(res) = memo[m][n] {
            return res;
        }

        let res = if m == 0 {
            Self::dp(m, n - 1, grid, memo) // 第一行：只能从左来
        } else if n == 0 {
            Self::dp(m - 1, n, grid, memo) // 第一列：只能从上来
        } else {
            Self::dp(m - 1, n, grid, memo) + Self::dp(m, n - 1, grid, memo)
        };

        memo[m][n] = Some(res);
        res
    }
}

// @lc code=end
