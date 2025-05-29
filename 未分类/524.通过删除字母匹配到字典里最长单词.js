/*
 * @lc app=leetcode.cn id=524 lang=javascript
 *
 * [524] 通过删除字母匹配到字典里最长单词
 *
 * https://leetcode.cn/problems/longest-word-in-dictionary-through-deleting/description/
 *
 * algorithms
 * Medium (50.48%)
 * Likes:    380
 * Dislikes: 0
 * Total Accepted:    113.4K
 * Total Submissions: 224.6K
 * Testcase Example:  '"abpcplea"\n["ale","apple","monkey","plea"]'
 *
 * 给你一个字符串 s 和一个字符串数组 dictionary ，找出并返回 dictionary 中最长的字符串，该字符串可以通过删除 s
 * 中的某些字符得到。
 *
 * 如果答案不止一个，返回长度最长且字母序最小的字符串。如果答案不存在，则返回空字符串。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "abpcplea", dictionary = ["ale","apple","monkey","plea"]
 * 输出："apple"
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = "abpcplea", dictionary = ["a","b","c"]
 * 输出："a"
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= s.length <= 1000
 * 1 <= dictionary.length <= 1000
 * 1 <= dictionary[i].length <= 1000
 * s 和 dictionary[i] 仅由小写英文字母组成
 *
 *
 */

// @lc code=start
/**
 * @param {string} s
 * @param {string[]} dictionary
 * @return {string}
 */
var findLongestWord = function (s, dictionary) {
    // 暴力法
    dictionary.sort((a, b) => {
        if (a.length !== b.length) {
            return b.length - a.length; // 长度不同，按长度降序排列
        } else {
            return a.localeCompare(b); // 长度相同，按字母序排列
        }
    });
    for (const word of dictionary) {
        let i = 0;
        let j = 0;
        while (i < s.length && j < word.length) {
            if (s[i] === word[j]) {
                j++;
            }
            i++;
        }
        if (j === word.length) {
            return word;
        }
    }
    return "";
};
// @lc code=end

// @after-stub-for-debug-begin
module.exports = findLongestWord;
// @after-stub-for-debug-end
