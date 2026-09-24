
/*
 * @lc app=leetcode.cn id=10 lang=rust
 *
 * [10] Regular Expression Matching
 *
 * https://leetcode.cn/problems/regular-expression-matching/description/
 *
 * algorithms
 * Hard (31.31%)
 * Likes:    4240
 * Dislikes: 0
 * Total Accepted:    509.8K
 * Total Submissions: 1.6M
 * Testcase Example:  '"aa"\n"a"'
 *
 * Given an input string s and a pattern p, implement regular expression
 * matching with support for '.' and '*' where:
 *
 *
 * '.' Matches any single character.​​​​
 * '*' Matches zero or more of the preceding element.
 *
 *
 * Return a boolean indicating whether the matching covers the entire input
 * string (not partial).
 *
 *
 * Example 1:
 *
 *
 * Input: s = "aa", p = "a"
 * Output: false
 * Explanation: "a" does not match the entire string "aa".
 *
 *
 * Example 2:
 *
 *
 * Input: s = "aa", p = "a*"
 * Output: true
 * Explanation: '*' means zero or more of the preceding element, 'a'.
 * Therefore, by repeating 'a' once, it becomes "aa".
 *
 *
 * Example 3:
 *
 *
 * Input: s = "ab", p = ".*"
 * Output: true
 * Explanation: ".*" means "zero or more (*) of any character (.)".
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 20
 * 1 <= p.length <= 20
 * s contains only lowercase English letters.
 * p contains only lowercase English letters, '.', and '*'.
 * It is guaranteed for each appearance of the character '*', there will be a
 * previous valid character to match.
 *
 *
 */
use crate::Solution;
// @lc code=start
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut s = s.chars().collect::<Vec<char>>();
        let mut p = p.chars().collect::<Vec<char>>();

        let mut memo: Vec<Vec<Option<bool>>> = vec![vec![None; p.len() + 1]; s.len() + 1];

        Self::dp(&s, &p, 0, 0, &mut memo)
    }

    fn dp(s: &[char], p: &[char], i: usize, j: usize, memo: &mut Vec<Vec<Option<bool>>>) -> bool {
        if let Some(result) = memo[i][j] {
            return result;
        }

        let result;
        if j == p.len() {
            result = i == s.len();
        } else {
            let first_match = i < s.len() && (s[i] == p[j] || p[j] == '.');

            if j + 1 < p.len() && p[j + 1] == '*' {
                result = Self::dp(s, p, i, j + 2, memo)
                    || (first_match && Self::dp(s, p, i, j + 1, memo));
            } else {
                result = first_match && Self::dp(s, p, i + 1, j + 1, memo);
            }
        }
        memo[i][j] = Some(result);
        result
    }
}
// @lc code=end
