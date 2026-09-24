/*
 * @lc app=leetcode.cn id=62 lang=rust
 *
 * [62] Unique Paths
 *
 * https://leetcode.cn/problems/unique-paths/description/
 *
 * algorithms
 * Medium (70.48%)
 * Likes:    2384
 * Dislikes: 0
 * Total Accepted:    1.3M
 * Total Submissions: 1.8M
 * Testcase Example:  '3\n7'
 *
 * There is a robot on an m x n grid. The robot is initially located at the
 * top-left corner (i.e., grid[0][0]). The robot tries to move to the
 * bottom-right corner (i.e., grid[m - 1][n - 1]). The robot can only move
 * either down or right at any point in time.
 *
 * Given the two integers m and n, return the number of possible unique paths
 * that the robot can take to reach the bottom-right corner.
 *
 * The test cases are generated so that the answer will be less than or equal
 * to 2 * 10^9.
 *
 *
 * Example 1:
 *
 *
 * Input: m = 3, n = 7
 * Output: 28
 *
 *
 * Example 2:
 *
 *
 * Input: m = 3, n = 2
 * Output: 3
 * Explanation: From the top-left corner, there are a total of 3 ways to reach
 * the bottom-right corner:
 * 1. Right -> Down -> Down
 * 2. Down -> Down -> Right
 * 3. Down -> Right -> Down
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= m, n <= 100
 *
 *
 */
use crate::Solution;

// @lc code=start
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut memo: Vec<Vec<Option<i32>>> = vec![vec![None; n as usize]; m as usize];
        Self::dp((m - 1) as usize, (n - 1) as usize, &mut memo)
    }
    // [m][n] = [m-1][n] + [m][n-1]
    fn dp(m: usize, n: usize, memo: &mut Vec<Vec<Option<i32>>>) -> i32 {
        if m == 0 || n == 0 {
            return 1;
        }
        if let Some(res) = memo[m][n] {
            return res;
        }

        let res = Self::dp(m - 1, n, memo) + Self::dp(m, n - 1, memo);
        memo[m][n] = Some(res);
        res
    }
}
// @lc code=end
