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

    const stack = [];
    const is_in_stack = new Map();
    for (let char of s) {
        count[char]--; // 每次遍历都递减计数

        if (is_in_stack.has(char)) {
            continue;
        }

        while (
            stack.length > 0 &&
            char < stack[stack.length - 1] &&
            count[stack[stack.length - 1]] > 0 // 确保栈顶字符后续还会出现
        ) {
            is_in_stack.delete(stack.pop()); // 正确使用Map的delete方法
        }

        stack.push(char);
        is_in_stack.set(char, true); // 正确使用Map的set方法
    }
    return stack.join("");
};
// @lc code=end
