/*
 * @lc app=leetcode.cn id=131 lang=rust
 *
 * [131] Palindrome Partitioning
 *
 * https://leetcode.cn/problems/palindrome-partitioning/description/
 *
 * algorithms
 * Medium (75.07%)
 * Likes:    2241
 * Dislikes: 0
 * Total Accepted:    815.4K
 * Total Submissions: 1.1M
 * Testcase Example:  '"aab"'
 *
 * Given a string s, partition s such that every substring of the partition is
 * a palindrome. Return all possible palindrome partitioning of s.
 * 
 * 
 * Example 1:
 * Input: s = "aab"
 * Output: [["a","a","b"],["aa","b"]]
 * Example 2:
 * Input: s = "a"
 * Output: [["a"]]
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= s.length <= 16
 * s contains only lowercase English letters.
 * 
 * 
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut path = Vec::new();
        let mut res = Vec::new();

        Self::back_track131(&s, 0, &mut path, &mut res);
        res
    }

    fn back_track131(
        s: &str,
        start: usize,
        current:&mut Vec<String>,
        resule :&mut Vec<Vec<String>>){

            if start == s.len(){
                resule.push(current.clone());
                return;
            }
            for end in start..s.len(){
                if Self::is_palindrome131(s.as_bytes(),start,end){
                    current.push(s[start..=end].to_string());
                    Self::back_track131(s, end+1, current, resule);
                    current.pop();
                }

            }

    }

    fn is_palindrome131(s:&[u8],mut left:usize,mut right:usize)->bool{
        while left < right{
            if s[left] != s[right]{
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}
// @lc code=end

