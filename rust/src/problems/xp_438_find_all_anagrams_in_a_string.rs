use std::collections::HashMap;

/*
 * @lc app=leetcode.cn id=438 lang=rust
 *
 * [438] Find All Anagrams in a String
 *
 * https://leetcode.cn/problems/find-all-anagrams-in-a-string/description/
 *
 * algorithms
 * Medium (54.59%)
 * Likes:    1940
 * Dislikes: 0
 * Total Accepted:    1M
 * Total Submissions: 1.9M
 * Testcase Example:  '"cbaebabacd"\n"abc"'
 *
 * Given two strings s and p, return an array of all the start indices of p's
 * anagrams in s. You may return the answer in any order.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "cbaebabacd", p = "abc"
 * Output: [0,6]
 * Explanation:
 * The substring with start index = 0 is "cba", which is an anagram of "abc".
 * The substring with start index = 6 is "bac", which is an anagram of "abc".
 *
 *
 * Example 2:
 *
 *
 * Input: s = "abab", p = "ab"
 * Output: [0,1,2]
 * Explanation:
 * The substring with start index = 0 is "ab", which is an anagram of "ab".
 * The substring with start index = 1 is "ba", which is an anagram of "ab".
 * The substring with start index = 2 is "ab", which is an anagram of "ab".
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length, p.length <= 3 * 10^4
 * s and p consist of lowercase English letters.
 *
 *
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut vec_p = vec![0; 26];
        let mut result = vec![];

        if s.len() < p.len() {
            return vec![];
        }

        for i in p.chars() {
            vec_p[(i as u8 - 'a' as u8) as usize] += 1;
        }

        for i in 0..=s.len() - p.len() {
            let mut vec_s = vec![0; 26];
            for c in (s[i..(i + p.len())]).chars() {
                vec_s[(c as u8 - 'a' as u8) as usize] += 1;
            }
            if vec_s == vec_p {
                result.push(i as i32);
            }
        }
        result
    }
}
// @lc code=end
