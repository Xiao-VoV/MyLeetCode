/*
 * @lc app=leetcode.cn id=49 lang=rust
 *
 * [49] Group Anagrams
 *
 * https://leetcode.cn/problems/group-anagrams/description/
 *
 * algorithms
 * Medium (69.57%)
 * Likes:    2696
 * Dislikes: 0
 * Total Accepted:    1.6M
 * Total Submissions: 2.3M
 * Testcase Example:  '["eat","tea","tan","ate","nat","bat"]'
 *
 * Given an array of strings strs, group the anagrams together. You can return
 * the answer in any order.
 *
 *
 * Example 1:
 *
 *
 * Input: strs = ["eat","tea","tan","ate","nat","bat"]
 *
 * Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
 *
 * Explanation:
 *
 *
 * There is no string in strs that can be rearranged to form "bat".
 * The strings "nat" and "tan" are anagrams as they can be rearranged to form
 * each other.
 * The strings "ate", "eat", and "tea" are anagrams as they can be rearranged
 * to form each other.
 *
 *
 *
 * Example 2:
 *
 *
 * Input: strs = [""]
 *
 * Output: [[""]]
 *
 *
 * Example 3:
 *
 *
 * Input: strs = ["a"]
 *
 * Output: [["a"]]
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= strs.length <= 10^4
 * 0 <= strs[i].length <= 100
 * strs[i] consists of lowercase English letters.
 *
 *
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs {
            let mut chars = s.chars().collect::<Vec<char>>();
            chars.sort();

            map.entry(chars.iter().collect::<String>())
                .and_modify(|e| e.push(s.clone()))
                .or_insert(vec![s.clone()]);
        }

        map.into_values().collect()
    }
}
// @lc code=end
