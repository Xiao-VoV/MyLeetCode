use std::{fmt::Result, result};

/*
 * @lc app=leetcode.cn id=22 lang=rust
 *
 * [22] Generate Parentheses
 *
 * https://leetcode.cn/problems/generate-parentheses/description/
 *
 * algorithms
 * Medium (79.04%)
 * Likes:    4120
 * Dislikes: 0
 * Total Accepted:    1.3M
 * Total Submissions: 1.7M
 * Testcase Example:  '3'
 *
 * Given n pairs of parentheses, write a function to generate all combinations
 * of well-formed parentheses.
 *
 *
 * Example 1:
 * Input: n = 3
 * Output: ["((()))","(()())","(())()","()(())","()()()"]
 * Example 2:
 * Input: n = 1
 * Output: ["()"]
 *
 *
 * Constraints:
 *
 *
 * 1 <= n <= 8
 *
 *
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = vec![];
        let mut current = String::new();
        Self::back_track22(0, 0, &mut current, &mut result, n);
        result
    }

    fn back_track22(left: i32, right: i32, current: &mut String, result: &mut Vec<String>, n: i32) {
        if current.len() == (n * 2) as usize {
            result.push(current.clone());
            return;
        }

        if left < n {
            current.push('(');
            Self::back_track22(left + 1, right, current, result, n);
            current.pop();
        }

        if left > right {
                current.push(')');
                Self::back_track22(left, right+1, current, result, n);
                current.pop();
        }
    }
}
// @lc code=end
