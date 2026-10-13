/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 * [53] Maximum Subarray
 *
 * https://leetcode.cn/problems/maximum-subarray/description/
 *
 * algorithms
 * Medium (56.90%)
 * Likes:    7499
 * Dislikes: 0
 * Total Accepted:    2.8M
 * Total Submissions: 4.9M
 * Testcase Example:  '[-2,1,-3,4,-1,2,1,-5,4]'
 *
 * Given an integer array nums, find the subarray with the largest sum, and
 * return its sum.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
 * Output: 6
 * Explanation: The subarray [4,-1,2,1] has the largest sum 6.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [1]
 * Output: 1
 * Explanation: The subarray [1] has the largest sum 1.
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [5,4,-1,7,8]
 * Output: 23
 * Explanation: The subarray [5,4,-1,7,8] has the largest sum 23.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^5
 * -10^4 <= nums[i] <= 10^4
 *
 *
 *
 * Follow up: If you have figured out the O(n) solution, try coding another
 * solution using the divide and conquer approach, which is more subtle.
 *
 */
use crate::Solution;
// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut end = 0usize;

        let mut max = i32::MIN;
        let mut current_max = 0;

        while end < nums.len() {
            if nums[end] < current_max + nums[end] {
                current_max += nums[end];
                max = current_max.max(max);
                end += 1;
            } else {
                current_max = nums[end];
                max = current_max.max(max);
                end += 1;
            }
        }
        max
    }
}
// @lc code=end
