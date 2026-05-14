/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 *
 * https://leetcode.cn/problems/longest-substring-without-repeating-characters/description/
 *
 * algorithms
 * Medium (42.40%)
 * Likes:    11455
 * Dislikes: 0
 * Total Accepted:    4.2M
 * Total Submissions: 9.9M
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
 * 0 <= s.length <= 5 * 10^4
 * s consists of English letters, digits, symbols and spaces.
 *
 *
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut left, mut right) = (0, 1);
        let mut max_length = 0;

        if s.len() == 0 || s.len() == 1 {
            return s.len() as i32;
        }
        let s: Vec<char> = s.chars().collect();
        use std::collections::HashSet;
        let mut set: HashSet<char> = HashSet::new();
        set.insert(s[left].clone());
        while right < s.len() {
            if !set.contains(&s[right]) {
                set.insert(s[right].clone());
                max_length = max_length.max(set.len());
                right += 1;
            } else {
                while set.contains(&s[right]) {
                    set.remove(&s[left]);
                    left += 1;
                }
            }
        }
        max_length as i32
    }
}
// @lc code=end
