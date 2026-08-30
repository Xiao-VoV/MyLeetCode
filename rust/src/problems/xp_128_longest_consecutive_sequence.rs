/*
 * @lc app=leetcode.cn id=128 lang=rust
 *
 * [128] Longest Consecutive Sequence
 *
 * https://leetcode.cn/problems/longest-consecutive-sequence/description/
 *
 * algorithms
 * Medium (48.97%)
 * Likes:    2881
 * Dislikes: 0
 * Total Accepted:    1.5M
 * Total Submissions: 3.1M
 * Testcase Example:  '[100,4,200,1,3,2]'
 *
 * Given an unsorted array of integers nums, return the length of the longest
 * consecutive elements sequence.
 *
 * You must write an algorithm that runs in O(n) time.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [100,4,200,1,3,2]
 * Output: 4
 * Explanation: The longest consecutive elements sequence is [1, 2, 3, 4].
 * Therefore its length is 4.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [0,3,7,2,5,8,4,6,0,1]
 * Output: 9
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [1,0,1,2]
 * Output: 3
 *
 *
 *
 * Constraints:
 *
 *
 * 0 <= nums.length <= 10^5
 * -10^9 <= nums[i] <= 10^9
 *
 *
 */

use super::Solution;
// @lc code=start
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let nums = nums.into_iter().collect::<HashSet<i32>>();
        let mut max = 0;
        for i in &nums {
            if nums.contains(&(i - 1)) {
                continue;
            } else {
                let mut end = i + 1;
                while nums.contains(&end) {
                    end += 1
                }
                max = max.max(end - i)
            }
        }

        max
    }
}
// @lc code=end
