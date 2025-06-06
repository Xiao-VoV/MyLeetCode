/*
 * @lc app=leetcode.cn id=374 lang=javascript
 *
 * [374] 猜数字大小
 *
 * https://leetcode.cn/problems/guess-number-higher-or-lower/description/
 *
 * algorithms
 * Easy (52.74%)
 * Likes:    364
 * Dislikes: 0
 * Total Accepted:    209.5K
 * Total Submissions: 396.1K
 * Testcase Example:  '10\n6'
 *
 * 我们正在玩猜数字游戏。猜数字游戏的规则如下：
 *
 * 我会从 1 到 n 随机选择一个数字。 请你猜选出的是哪个数字。
 *
 * 如果你猜错了，我会告诉你，我选出的数字比你猜测的数字大了还是小了。
 *
 * 你可以通过调用一个预先定义好的接口 int guess(int num) 来获取猜测结果，返回值一共有三种可能的情况：
 *
 *
 * -1：你猜的数字比我选出的数字大 （即 num > pick）。
 * 1：你猜的数字比我选出的数字小 （即 num < pick）。
 * 0：你猜的数字与我选出的数字相等。（即 num == pick）。
 *
 *
 * 返回我选出的数字。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：n = 10, pick = 6
 * 输出：6
 *
 *
 * 示例 2：
 *
 *
 * 输入：n = 1, pick = 1
 * 输出：1
 *
 *
 * 示例 3：
 *
 *
 * 输入：n = 2, pick = 1
 * 输出：1
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= n <= 2^31 - 1
 * 1 <= pick <= n
 *
 *
 */

// @lc code=start
/**
 * Forward declaration of guess API.
 * @param {number} num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * var guess = function(num) {}
 */

/**
 * @param {number} n
 * @return {number}
 */
var guessNumber = function (n) {
    let left = 0;
    let right = n;
    while (left <= right) {
        let mid = Math.floor((left + right) / 2);

        if (guess(mid) === 0) {
            return mid;
        } else if (guess(mid) === -1) {
            right = mid - 1;
        } else if (guess(mid) === 1) {
            left = mid + 1;
        }
    }
    return left;
};
// @lc code=end
