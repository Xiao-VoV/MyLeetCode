/*
 * @lc app=leetcode.cn id=424 lang=javascript
 *
 * [424] 替换后的最长重复字符
 *
 * https://leetcode.cn/problems/longest-repeating-character-replacement/description/
 *
 * algorithms
 * Medium (55.45%)
 * Likes:    902
 * Dislikes: 0
 * Total Accepted:    105.7K
 * Total Submissions: 190.5K
 * Testcase Example:  '"ABAB"\n2'
 *
 * 给你一个字符串 s 和一个整数 k 。你可以选择字符串中的任一字符，并将其更改为任何其他大写英文字符。该操作最多可执行 k 次。
 *
 * 在执行上述操作后，返回 包含相同字母的最长子字符串的长度。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "ABAB", k = 2
 * 输出：4
 * 解释：用两个'A'替换为两个'B',反之亦然。
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = "AABABBA", k = 1
 * 输出：4
 * 解释：
 * 将中间的一个'A'替换为'B',字符串变为 "AABBBBA"。
 * 子串 "BBBB" 有最长重复字母, 答案为 4。
 * 可能存在其他的方法来得到同样的结果。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= s.length <= 10^5
 * s 仅由大写英文字母组成
 * 0 <= k <= s.length
 *
 *
 */

// @lc code=start
/**
 * @param {string} s
 * @param {number} k
 * @return {number}
 */
var characterReplacement = function (s, k) {
    let left = (right = 0);
    let maxCount = 0;
    let maxLength = 0;
    let count = new Array(26).fill(0);
    while (right < s.length) {
        count[s.charCodeAt(right) - "A".charCodeAt(0)]++;
        maxCount = Math.max(
            maxCount,
            count[s.charCodeAt(right) - "A".charCodeAt(0)]
        );
        if (right - left + 1 - maxCount > k) {
            count[s.charCodeAt(left) - "A".charCodeAt(0)]--;
            left++;
        }

        maxLength = Math.max(maxLength, right - left + 1);
        right++;
    }

    return maxLength;
};
// @lc code=end
