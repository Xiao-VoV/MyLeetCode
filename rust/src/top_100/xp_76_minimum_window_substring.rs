/*
 * @lc app=leetcode.cn id=76 lang=rust
 *
 * [76] Minimum Window Substring
 *
 * https://leetcode.cn/problems/minimum-window-substring/description/
 *
 * algorithms
 * Hard (49.66%)
 * Likes:    3705
 * Dislikes: 0
 * Total Accepted:    1.2M
 * Total Submissions: 2.4M
 * Testcase Example:  '"ADOBECODEBANC"\n"ABC"'
 *
 * Given two strings s and t of lengths m and n respectively, return the
 * minimum window substring of s such that every character in t (including
 * duplicates) is included in the window. If there is no such substring, return
 * the empty string "".
 *
 * The testcases will be generated such that the answer is unique.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "ADOBECODEBANC", t = "ABC"
 * Output: "BANC"
 * Explanation: The minimum window substring "BANC" includes 'A', 'B', and 'C'
 * from string t.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "a", t = "a"
 * Output: "a"
 * Explanation: The entire string s is the minimum window.
 *
 *
 * Example 3:
 *
 *
 * Input: s = "a", t = "aa"
 * Output: ""
 * Explanation: Both 'a's from t must be included in the window.
 * Since the largest window of s only has one 'a', return empty string.
 *
 *
 *
 * Constraints:
 *
 *
 * m == s.length
 * n == t.length
 * 1 <= m, n <= 10^5
 * s and t consist of uppercase and lowercase English letters.
 *
 *
 *
 * Follow up: Could you find an algorithm that runs in O(m + n) time?
 *
 */
use crate::Solution;
use std::collections::HashMap;
// @lc code=start
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut minlen = usize::MAX;
        let mut index = (0, 0);
        let mut left = 0;
        let mut right = 0;

        let s: Vec<char> = s.chars().collect();
        let t = t.chars().collect::<Vec<char>>();

        let mut HashT = HashMap::new();
        for c in t {
            *HashT.entry(c).or_insert(0) += 1;
        }

        let mut HashS = HashMap::new();
        while right < s.len() {
            *HashS.entry(s[right]).or_insert(0) += 1;
            while Self::check(&HashS, &HashT) {
                if minlen > right - left + 1 {
                    index = (left, right);
                    minlen = right - left + 1;
                }
                HashS.entry(s[left]).and_modify(|count| *count -= 1);
                left += 1;
            }
            right += 1;
        }

        if minlen == usize::MAX {
            "".to_string()
        } else {
            s[index.0..=index.1].iter().collect()
        }
    }

    fn check(HashS: &HashMap<char, i32>, HashT: &HashMap<char, i32>) -> bool {
        for (k, v) in HashT {
            let count_s = HashS.get(&k).copied().unwrap_or(0);
            if count_s < *v {
                return false;
            }
        }
        true
    }
}
// @lc code=end
