/*
 * @lc app=leetcode.cn id=118 lang=rust
 *
 * [118] Pascal's Triangle
 *
 * https://leetcode.cn/problems/pascals-triangle/description/
 *
 * algorithms
 * Easy (77.90%)
 * Likes:    1380
 * Dislikes: 0
 * Total Accepted:    888.1K
 * Total Submissions: 1.1M
 * Testcase Example:  '5'
 *
 * Given an integer numRows, return the first numRows of Pascal's triangle.
 *
 * In Pascal's triangle, each number is the sum of the two numbers directly
 * above it as shown:
 *
 *
 * Example 1:
 * Input: numRows = 5
 * Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
 * Example 2:
 * Input: numRows = 1
 * Output: [[1]]
 *
 *
 * Constraints:
 *
 *
 * 1 <= numRows <= 30
 *
 *
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        Self::dfs_118(num_rows, &mut result);

        result
    }

    fn dfs_118(rows: i32, result: &mut Vec<Vec<i32>>) {
        if rows == 1 {
            result.push(vec![1]);
            return;
        }

        Self::dfs_118(rows - 1, result);

        let last_row = result.last().unwrap();
        let mut new_row = vec![1];

        for i in 0..last_row.len() - 1 {
            new_row.push(last_row[i] + last_row[i + 1]);
        }

        new_row.push(1);
        result.push(new_row);
    }
}
// @lc code=end
