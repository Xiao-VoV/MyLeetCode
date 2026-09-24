/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 *
 * https://leetcode.cn/problems/longest-palindromic-substring/description/
 *
 * algorithms
 * Medium (41.17%)
 * Likes:    8182
 * Dislikes: 0
 * Total Accepted:    2.5M
 * Total Submissions: 6M
 * Testcase Example:  '"babad"'
 *
 * Given a string s, return the longest palindromic substring in s.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "babad"
 * Output: "bab"
 * Explanation: "aba" is also a valid answer.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "cbbd"
 * Output: "bb"
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 1000
 * s consist of only digits and English letters.
 *
 *
 */

use std::result;

use crate::Solution;
// @lc code=start
impl Solution {
    pub fn longest_palindrome(mut s: String) -> String {
        let s: Vec<char> = s.chars().collect();

        let n = s.len();
        if n < 2 {
            return s.iter().collect();
        }

        let mut meno: Vec<Vec<Option<bool>>> = vec![vec![None; n]; n];

        let mut start = 0;
        let mut max_len = 1;

        for len in 1..=n {
            for i in 0..=n - len {
                let j = i + len - 1;
                if Self::is_palindrome(&s, i, j, &mut meno) && len > max_len {
                    start = i;
                    max_len = len;
                }
            }
        }

        s[start..start + max_len].iter().collect()
    }

    fn is_palindrome(s: &[char], i: usize, j: usize, memo: &mut Vec<Vec<Option<bool>>>) -> bool {
        if i >= j {
            return true;
        }
        if let Some(result) = memo[i][j] {
            return result;
        }

        let result = s[i] == s[j] && Self::is_palindrome(s, i + 1, j - 1, memo);
        memo[i][j] = Some(result);

        result
    }
}
// @lc code=end
