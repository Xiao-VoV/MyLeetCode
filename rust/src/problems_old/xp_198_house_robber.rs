/*
 * @lc app=leetcode.cn id=198 lang=rust
 *
 * [198] House Robber
 *
 * https://leetcode.cn/problems/house-robber/description/
 *
 * algorithms
 * Medium (56.53%)
 * Likes:    3554
 * Dislikes: 0
 * Total Accepted:    1.7M
 * Total Submissions: 2.9M
 * Testcase Example:  '[1,2,3,1]'
 *
 * You are a professional robber planning to rob houses along a street. Each
 * house has a certain amount of money stashed, the only constraint stopping
 * you from robbing each of them is that adjacent houses have security systems
 * connected and it will automatically contact the police if two adjacent
 * houses were broken into on the same night.
 *
 * Given an integer array nums representing the amount of money of each house,
 * return the maximum amount of money you can rob tonight without alerting the
 * police.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,2,3,1]
 * Output: 4
 * Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
 * Total amount you can rob = 1 + 3 = 4.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [2,7,9,3,1]
 * Output: 12
 * Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house
 * 5 (money = 1).
 * Total amount you can rob = 2 + 9 + 1 = 12.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 100
 * 0 <= nums[i] <= 400
 *
 *
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {

        let mut pre2 = 0;
        let mut pre1 = 0;

        for num in nums{
            
            let cur = std::cmp::max(pre1, pre2 + num);
            pre2 = pre1;
            pre1 = cur;
        }

        pre1

    }
}
// @lc code=end
