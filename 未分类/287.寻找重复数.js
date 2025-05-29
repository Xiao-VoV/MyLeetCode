/*
 * @lc app=leetcode.cn id=287 lang=javascript
 *
 * [287] 寻找重复数
 *
 * https://leetcode.cn/problems/find-the-duplicate-number/description/
 *
 * algorithms
 * Medium (65.34%)
 * Likes:    2529
 * Dislikes: 0
 * Total Accepted:    449.2K
 * Total Submissions: 687.4K
 * Testcase Example:  '[1,3,4,2,2]'
 *
 * 给定一个包含 n + 1 个整数的数组 nums ，其数字都在 [1, n] 范围内（包括 1 和 n），可知至少存在一个重复的整数。
 *
 * 假设 nums 只有 一个重复的整数 ，返回 这个重复的数 。
 *
 * 你设计的解决方案必须 不修改 数组 nums 且只用常量级 O(1) 的额外空间。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,3,4,2,2]
 * 输出：2
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [3,1,3,4,2]
 * 输出：3
 *
 *
 * 示例 3 :
 *
 *
 * 输入：nums = [3,3,3,3,3]
 * 输出：3
 *
 *
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= n <= 10^5
 * nums.length == n + 1
 * 1 <= nums[i] <= n
 * nums 中 只有一个整数 出现 两次或多次 ，其余整数均只出现 一次
 *
 *
 *
 *
 * 进阶：
 *
 *
 * 如何证明 nums 中至少存在一个重复的数字?
 * 你可以设计一个线性级时间复杂度 O(n) 的解决方案吗？
 *
 *
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number}
 */
var findDuplicate = function (nums) {
    // for (let [key1, val1] of nums.entries()) {
    //     for (let [key2, val2] of nums.entries()) {
    //         if (key1 != key2 && val1 === val2) {
    //             return val2;
    //         }
    //     }
    // }
    //将数组中的每个数字看作是一个节点，数字的值表示下一个节点的索引。由于数组中存在重复的数字，那么必然存在一个环。我们可以使用快慢指针来找到这个环，并确定环的入口节点，这个入口节点就是重复的数字
    let i = nums[0];
    let j = nums[0];
    do {
        i = nums[i];
        j = nums[nums[j]];
    } while (i !== j);
    i = nums[0];
    while (i !== j) {
        i = nums[i];
        j = nums[j];
    }

    return i;
};
// @lc code=end

// @after-stub-for-debug-begin
module.exports = findDuplicate;
// @after-stub-for-debug-end
