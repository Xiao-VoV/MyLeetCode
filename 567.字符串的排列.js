/*
 * @lc app=leetcode.cn id=567 lang=javascript
 *
 * [567] 字符串的排列
 *
 * https://leetcode.cn/problems/permutation-in-string/description/
 *
 * algorithms
 * Medium (45.58%)
 * Likes:    1044
 * Dislikes: 0
 * Total Accepted:    313K
 * Total Submissions: 685.1K
 * Testcase Example:  '"ab"\n"eidbaooo"'
 *
 * 给你两个字符串 s1 和 s2 ，写一个函数来判断 s2 是否包含 s1 的 排列。如果是，返回 true ；否则，返回 false 。
 *
 * 换句话说，s1 的排列之一是 s2 的 子串 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s1 = "ab" s2 = "eidbaooo"
 * 输出：true
 * 解释：s2 包含 s1 的排列之一 ("ba").
 *
 *
 * 示例 2：
 *
 *
 * 输入：s1= "ab" s2 = "eidboaoo"
 * 输出：false
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= s1.length, s2.length <= 10^4
 * s1 和 s2 仅包含小写字母
 *
 *
 */

// @lc code=start
/**
 * @param {string} s1
 * @param {string} s2
 * @return {boolean}
 */
var checkInclusion = function (s1, s2) {
    if (s1.length > s2.length) {
        return false;
    }
    let target = new Array(26).fill(0);
    let window = new Array(26).fill(0);

    for (let i = 0; i < s1.length; i++) {
        const index = s1.charCodeAt(i) - "a".charCodeAt(0);
        target[index]++;
    }

    let left = 0,
        right = 0;
    while (right < s2.length) {
        const rightIndex = s2.charCodeAt(right) - "a".charCodeAt(0);
        window[rightIndex]++;
        right++;
        while (right - left > s1.length) {
            const leftIndex = s2.charCodeAt(left) - "a".charCodeAt(0);
            window[leftIndex]--;
            left++;
        }
        if (arrayEqual(target, window)) {
            return true;
        }
    }
    return false;
};

function arrayEqual(a, b) {
    for (let i = 0; i < a.length; i++) {
        if (a[i] !== b[i]) {
            return false;
        }
    }
    return true;
}
// @lc code=end

// @after-stub-for-debug-begin
module.exports = checkInclusion;
// @after-stub-for-debug-end
