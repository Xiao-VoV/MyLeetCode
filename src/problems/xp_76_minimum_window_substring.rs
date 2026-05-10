use std::clone;

/*
 * @lc app=leetcode.cn id=76 lang=rust
 *
 * [76] Minimum Window Substring
 *
 * https://leetcode.cn/problems/minimum-window-substring/description/
 *
 * algorithms
 * Hard (49.24%)
 * Likes:    3594
 * Dislikes: 0
 * Total Accepted:    1.1M
 * Total Submissions: 2.2M
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
use super::Solution;
// @lc code=start
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() == 0 || s.len() < t.len() {
            return "".to_string();
        }

        let mut vec_t = vec![0; 128];
        for c in t.chars() {
            let i = c as u8 - 'a' as u8;
            vec_t[i as usize] += 1;
        }

        let mut vec_s = vec![0; 128];
        let vecs_char = s.chars().collect::<Vec<char>>();
        for i in 0..t.len() {
            let j = vecs_char[i] as u8 - 'a' as u8;
            vec_s[j as usize] += 1;
        }
        if vec_s == vec_t {
            return t;
        }

        let mut left = 0;
        let mut right = t.len();

        let min = t.len();
        let mut result = String::new();
        while right < s.len() {
            right += 1;
            let index = vecs_char[right] as u8 - 'a' as u8;
            vec_s[index as usize] += 1;

            while Self::contain(&vec_s[left..right], &vec_t) {
                result = String::from_iter(vecs_char[left..right].to_owned().iter());
                
                left +=1;
            }
        }

        todo!()
    }

    fn contain(a: &[i32], b: &Vec<i32>) -> bool {
        for (i,v) in b.iter().enumerate(){
            if a[i] < *v
            {
                return false;
            }
        }
        true
    }
}
// @lc code=end
