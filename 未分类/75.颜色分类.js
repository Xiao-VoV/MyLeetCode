/*
 * @lc app=leetcode.cn id=75 lang=javascript
 *
 * [75] 颜色分类
 *
 * https://leetcode.cn/problems/sort-colors/description/
 *
 * algorithms
 * Medium (62.04%)
 * Likes:    1868
 * Dislikes: 0
 * Total Accepted:    716K
 * Total Submissions: 1.2M
 * Testcase Example:  '[2,0,2,1,1,0]'
 *
 * 给定一个包含红色、白色和蓝色、共 n 个元素的数组 nums ，原地 对它们进行排序，使得相同颜色的元素相邻，并按照红色、白色、蓝色顺序排列。
 *
 * 我们使用整数 0、 1 和 2 分别表示红色、白色和蓝色。
 *
 *
 *
 *
 * 必须在不使用库内置的 sort 函数的情况下解决这个问题。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [2,0,2,1,1,0]
 * 输出：[0,0,1,1,2,2]
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [2,0,1]
 * 输出：[0,1,2]
 *
 *
 *
 *
 * 提示：
 *
 *
 * n == nums.length
 * 1 <= n <= 300
 * nums[i] 为 0、1 或 2
 *
 *
 *
 *
 * 进阶：
 *
 *
 * 你能想出一个仅使用常数空间的一趟扫描算法吗？
 *
 *
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {void} Do not return anything, modify nums in-place instead.
 */
var sortColors = function (nums) {
    // nums.sort((a, b) => a - b);
    let left = 0,
        right = nums.length - 1,
        i = 0;
    while (i <= right) {
        if (nums[i] === 2) {
            [nums[i], nums[right]] = [nums[right], nums[i]];
            right--;
        } else if (nums[i] === 0) {
            [nums[i], nums[left]] = [nums[left], nums[i]];
            left++;
            i++;
        } else {
            i++;
        }
    }
};
// @lc code=end

// @after-stub-for-debug-begin
module.exports = sortColors;
// @after-stub-for-debug-end
