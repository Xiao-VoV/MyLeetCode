/*
 * @lc app=leetcode.cn id=70 lang=rust
 *
 * [70] Climbing Stairs
 *
 * https://leetcode.cn/problems/climbing-stairs/description/
 *
 * algorithms
 * Easy (55.79%)
 * Likes:    4104
 * Dislikes: 0
 * Total Accepted:    2.3M
 * Total Submissions: 4M
 * Testcase Example:  '2'
 *
 * You are climbing a staircase. It takes n steps to reach the top.
 *
 * Each time you can either climb 1 or 2 steps. In how many distinct ways can
 * you climb to the top?
 *
 *
 * Example 1:
 *
 *
 * Input: n = 2
 * Output: 2
 * Explanation: There are two ways to climb to the top.
 * 1. 1 step + 1 step
 * 2. 2 steps
 *
 *
 * Example 2:
 *
 *
 * Input: n = 3
 * Output: 3
 * Explanation: There are three ways to climb to the top.
 * 1. 1 step + 1 step + 1 step
 * 2. 1 step + 2 steps
 * 3. 2 steps + 1 step
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= n <= 45
 *
 *
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut memo = vec![0; (n + 1) as usize];

        Self::dfc_70(n, &mut memo)
    }

    fn dfc_70(n: i32, memo: &mut Vec<i32>) -> i32 {
        let index = n as usize;
        if n <= 2 {
            return n;
        }

        if memo[index] != 0 {
            return memo[index];
        }

        memo[index] = Self::dfc_70(n - 1, memo) + Self::dfc_70(n - 2, memo);

        memo[index]
    }
}
// @lc code=end
