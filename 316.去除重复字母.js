/*
 * @lc app=leetcode.cn id=316 lang=javascript
 *
 * [316] 去除重复字母
 *
 * https://leetcode.cn/problems/remove-duplicate-letters/description/
 *
 * algorithms
 * Medium (49.59%)
 * Likes:    1140
 * Dislikes: 0
 * Total Accepted:    158K
 * Total Submissions: 317.2K
 * Testcase Example:  '"bcabc"'
 *
 * 给你一个字符串 s ，请你去除字符串中重复的字母，使得每个字母只出现一次。需保证 返回结果的字典序最小（要求不能打乱其他字符的相对位置）。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "bcabc"
 * 输出："abc"
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = "cbacdcbc"
 * 输出："acdb"
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= s.length <= 10^4
 * s 由小写英文字母组成
 *
 *
 *
 *
 * 注意：该题与 1081
 * https://leetcode-cn.com/problems/smallest-subsequence-of-distinct-characters
 * 相同
 *
 */

// @lc code=start
/**
 * @param {string} s
 * @return {string}
 */
var removeDuplicateLetters = function (s) {
    const count = {};
    for (let char of s) {
        count[char] = (count[char] || 0) + 1;
    }
    // console.log(count);
    const is_in_stack = {};
    const stack = [];
    for (let char of s) {
        count[char]--;
        if (is_in_stack[char]) continue;

        while (
            stack.length > 0 &&
            stack[stack.length - 1] > char &&
            count[stack[stack.length - 1]] > 0
        ) {
            const popped = stack.pop();
            is_in_stack[popped] = false;
        }
        stack.push(char);
        is_in_stack[char] = true;
    }
    return stack.join("");
};
// @lc code=end
