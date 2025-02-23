/*
 * @lc app=leetcode.cn id=532 lang=javascript
 *
 * [532] 数组中的 k-diff 数对
 *
 * https://leetcode.cn/problems/k-diff-pairs-in-an-array/description/
 *
 * algorithms
 * Medium (46.57%)
 * Likes:    293
 * Dislikes: 0
 * Total Accepted:    69.7K
 * Total Submissions: 149.6K
 * Testcase Example:  '[3,1,4,1,5]\n2'
 *
 * 给你一个整数数组 nums 和一个整数 k，请你在数组中找出 不同的 k-diff 数对，并返回不同的 k-diff 数对 的数目。
 *
 * k-diff 数对定义为一个整数对 (nums[i], nums[j]) ，并满足下述全部条件：
 *
 *
 * 0 <= i, j < nums.length
 * i != j
 * |nums[i] - nums[j]| == k
 *
 *
 * 注意，|val| 表示 val 的绝对值。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [3, 1, 4, 1, 5], k = 2
 * 输出：2
 * 解释：数组中有两个 2-diff 数对, (1, 3) 和 (3, 5)。
 * 尽管数组中有两个 1 ，但我们只应返回不同的数对的数量。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [1, 2, 3, 4, 5], k = 1
 * 输出：4
 * 解释：数组中有四个 1-diff 数对, (1, 2), (2, 3), (3, 4) 和 (4, 5) 。
 *
 *
 * 示例 3：
 *
 *
 * 输入：nums = [1, 3, 1, 5, 4], k = 0
 * 输出：1
 * 解释：数组中只有一个 0-diff 数对，(1, 1) 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 10^4
 * -10^7 <= nums[i] <= 10^7
 * 0 <= k <= 10^7
 *
 *
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
var findPairs = function (nums, k) {
    // let count = new Map();
    // let result = 0;
    // // 统计每个数字出现的次数
    // for (let num of nums) {
    //     count.set(num, (count.get(num) || 0) + 1);
    // }
    // // 遍历数组，检查每个数字是否存在 k-diff 数对
    // for (let num of nums) {
    //     if (k > 0 && count.has(num + k)) {
    //         result++;
    //     } else if (k == 0 && count.get(num) > 1) {
    //         result++;
    //     }
    // }
    // return result;

    // 首先进行排序
    nums.sort((a, b) => a - b);
    let left = 0,
        right = 1 > nums.length ? nums.length : 1,
        result = 0;
    while (right < nums.length) {
        if (left == right) {
            right++;
            continue;
        }
        if (nums[right] - nums[left] === k) {
            result++;
            while (left + 1 < nums.length && nums[left] === nums[left + 1]) {
                left++;
            }
            left++;
            // 跳过相同的 right 元素
            while (right + 1 < nums.length && nums[right] === nums[right + 1]) {
                right++;
            }
            right++;
        } else if (nums[right] - nums[left] < k) {
            right++;
        } else if (nums[right] - nums[left] > k) {
            left++;
        }
    }
    return result;
};
// @lc code=end

// @after-stub-for-debug-begin
module.exports = findPairs;
// @after-stub-for-debug-end
