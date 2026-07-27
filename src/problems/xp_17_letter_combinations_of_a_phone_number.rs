/*
 * @lc app=leetcode.cn id=17 lang=rust
 *
 * [17] Letter Combinations of a Phone Number
 *
 * https://leetcode.cn/problems/letter-combinations-of-a-phone-number/description/
 *
 * algorithms
 * Medium (63.80%)
 * Likes:    3278
 * Dislikes: 0
 * Total Accepted:    1.4M
 * Total Submissions: 2.2M
 * Testcase Example:  ""23""
 *
 * Given a string containing digits from 2-9 inclusive, return all possible
 * letter combinations that the number could represent. Return the answer in
 * any order.
 *
 * A mapping of digits to letters (just like on the telephone buttons) is given
 * below. Note that 1 does not map to any letters.
 *
 *
 * Example 1:
 *
 *
 * Input: digits = "23"
 * Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
 *
 *
 * Example 2:
 *
 *
 * Input: digits = "2"
 * Output: ["a","b","c"]
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= digits.length <= 4
 * digits[i] is a digit in the range ["2", "9"].
 *
 *
 */

use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        use std::collections::HashMap;

        let PHONENUMBER: HashMap<String, Vec<&str>> = HashMap::from([
            ("2".to_string(), vec!["a", "b", "c"]),
            ("3".to_string(), vec!["d", "e", "f"]),
            ("4".to_string(), vec!["g", "h", "i"]),
            ("5".to_string(), vec!["j", "k", "l"]),
            ("6".to_string(), vec!["m", "n", "o"]),
            ("7".to_string(), vec!["p", "q", "r", "s"]),
            ("8".to_string(), vec!["t", "u", "v"]),
            ("9".to_string(), vec!["w", "x", "y", "z"]),
        ]);

        let mut Result = Vec::new();
        let mut current = String::new();
        Self::back_tracking17(&digits, &mut current, &mut Result, 0, &PHONENUMBER);
        Result
    }
    fn back_tracking17(
        digits: &str,
        current: &mut String,
        result: &mut Vec<String>,
        start: usize,
        PHONENUMBER: &HashMap<String, Vec<&str>>,
    ) {
        if digits.len() == 0 {
            result.push(current.clone());
            return;
        }
        let first = &digits[0..1];
        let rest = &digits[1..];
        let letters = PHONENUMBER.get(first).unwrap();
        for i in 0..letters.len() {
            current.push_str(letters[i]);
            Self::back_tracking17(rest, current, result, 0,PHONENUMBER);
            current.pop();
        }
    }
}
// @lc code=end
