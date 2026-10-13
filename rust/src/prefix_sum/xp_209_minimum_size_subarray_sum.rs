use std::cmp::min;

/*
 * @lc app=leetcode.cn id=209 lang=rust
 *
 * [209] Minimum Size Subarray Sum
 *
 * https://leetcode.cn/problems/minimum-size-subarray-sum/description/
 *
 * algorithms
 * Medium (47.55%)
 * Likes:    2746
 * Dislikes: 0
 * Total Accepted:    1.3M
 * Total Submissions: 2.7M
 * Testcase Example:  '7\n[2,3,1,2,4,3]'
 *
 * Given an array of positive integers nums and a positive integer target,
 * return the minimal length of a subarray whose sum is greater than or equal
 * to target. If there is no such subarray, return 0 instead.
 *
 *
 * Example 1:
 *
 *
 * Input: target = 7, nums = [2,3,1,2,4,3]
 * Output: 2
 * Explanation: The subarray [4,3] has the minimal length under the problem
 * constraint.
 *
 *
 * Example 2:
 *
 *
 * Input: target = 4, nums = [1,4,4]
 * Output: 1
 *
 *
 * Example 3:
 *
 *
 * Input: target = 11, nums = [1,1,1,1,1,1,1,1]
 * Output: 0
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= target <= 10^9
 * 1 <= nums.length <= 10^5
 * 1 <= nums[i] <= 10^4
 *
 *
 *
 * Follow up: If you have figured out the O(n) solution, try coding another
 * solution of which the time complexity is O(n log(n)).
 */
use crate::Solution;
// @lc code=start
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut prefix = vec![0; nums.len() + 1];
        for (k, v) in nums.iter().enumerate() {
            prefix[k + 1] = prefix[k] + nums[k];
        }

        let mut min_len = i32::MAX;

        for i in 0..nums.len() {
            let want = prefix[i] + target;

            let pos = prefix[i + 1..].partition_point(|&v| v < want) + i + 1;

            if pos <= nums.len() {
                min_len = min_len.min((pos - i) as i32)
            }
        }

        if min_len == i32::MAX { 0 } else { min_len }
    }
}
// @lc code=end
