use std::{
    collections::{HashMap, VecDeque},
    result,
};

/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 *
 * https://leetcode.cn/problems/longest-substring-without-repeating-characters/description/
 *
 * algorithms
 * Medium (42.88%)
 * Likes:    11619
 * Dislikes: 0
 * Total Accepted:    4.4M
 * Total Submissions: 10.4M
 * Testcase Example:  '"abcabcbb"'
 *
 * Given a string s, find the length of the longest substring without duplicate
 * characters.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3. Note that "bca" and
 * "cab" are also correct answers.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "bbbbb"
 * Output: 1
 * Explanation: The answer is "b", with the length of 1.
 *
 *
 * Example 3:
 *
 *
 * Input: s = "pwwkew"
 * Output: 3
 * Explanation: The answer is "wke", with the length of 3.
 * Notice that the answer must be a substring, "pwke" is a subsequence and not
 * a substring.
 *
 *
 *
 * Constraints:
 *
 *
 * 0 <= s.length <= 10^5
 * s consists of English letters, digits, symbols and spaces.
 *
 *
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        use std::collections::HashMap;
        let mut count = HashMap::<char, usize>::new();

        let mut left = 0;

        let mut result = 0;
        for (right, c) in s.iter().enumerate() {
            if let Some(&prev) = count.get(c) {
                left = left.max(prev + 1);
            }
            result = result.max(right -left+1);
            count.insert(*c, right);
        }

        result as i32
    }
}
// @lc code=end
