/*
 * @lc app=leetcode.cn id=338 lang=rust
 *
 * [338] Counting Bits
 *
 * https://leetcode.cn/problems/counting-bits/description/
 *
 * algorithms
 * Easy (78.97%)
 * Likes:    1444
 * Dislikes: 0
 * Total Accepted:    410.7K
 * Total Submissions: 520.1K
 * Testcase Example:  '2'
 *
 * Given an integer n, return an array ans of length n + 1 such that for each i
 * (0 <= i <= n), ans[i] is the number of 1's in the binary representation of
 * i.
 * Do not solve it with built-in functions (i.e., like __builtin_popcount in
 * C++).
 * 
 * Example 1:
 * 
 * 
 * Input: n = 2
 * Output: [0,1,1]
 * Explanation:
 * 0 --> 0
 * 1 --> 1
 * 2 --> 10
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: n = 5
 * Output: [0,1,1,2,1,2]
 * Explanation:
 * 0 --> 0
 * 1 --> 1
 * 2 --> 10
 * 3 --> 11
 * 4 --> 100
 * 5 --> 101
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 0 <= n <= 10^5
 * 
 * 
 * 
 * Follow up:
 * 
 * 
 * It is very easy to come up with a solution with a runtime of O(n log n). Can
 * you do it in linear time O(n) and possibly in a single pass?
 * 
 * 
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut result = Vec::new(); 
        for i in 0..=n{
            result.push(i.count_ones() as i32);
        }

        result
    }
}
// @lc code=end

