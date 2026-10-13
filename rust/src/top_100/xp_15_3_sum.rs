use core::num;

/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 3Sum
 *
 * https://leetcode.cn/problems/3sum/description/
 *
 * algorithms
 * Medium (40.63%)
 * Likes:    8212
 * Dislikes: 0
 * Total Accepted:    3.1M
 * Total Submissions: 7.7M
 * Testcase Example:  '[-1,0,1,2,-1,-4]'
 *
 * Given an integer array nums, return all the triplets [nums[i], nums[j],
 * nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] +
 * nums[k] == 0.
 *
 * Notice that the solution set must not contain duplicate triplets.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [-1,0,1,2,-1,-4]
 * Output: [[-1,-1,2],[-1,0,1]]
 * Explanation:
 * nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
 * nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
 * nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
 * The distinct triplets are [-1,0,1] and [-1,-1,2].
 * Notice that the order of the output and the order of the triplets does not
 * matter.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [0,1,1]
 * Output: []
 * Explanation: The only possible triplet does not sum up to 0.
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [0,0,0]
 * Output: [[0,0,0]]
 * Explanation: The only possible triplet sums up to 0.
 *
 *
 *
 * Constraints:
 *
 *
 * 3 <= nums.length <= 3000
 * -10^5 <= nums[i] <= 10^5
 *
 *
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut left = 1usize;
        let mut right = nums.len() - 1;

        let mut pin = 0usize;

        nums.sort_unstable();

        let mut result = Vec::<Vec<i32>>::new();

        while pin < nums.len() {
            if pin > 0 && nums[pin] == nums[pin - 1] {
                pin += 1;
                continue;
            }
            left = pin + 1;
            right = nums.len() - 1;

            while left < right {
                if (nums[left] + nums[right] + nums[pin]) == 0 {
                    result.push(vec![nums[left], nums[right], nums[pin]]);

                    // left 跳过重复
                    left += 1;
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }

                    // right 跳过重复
                    right -= 1;
                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                } else if (nums[left] + nums[right] + nums[pin]) > 0 {
                    right -= 1;
                } else {
                    left += 1;
                }
            }

            pin += 1;
        }

        result
    }
}
// @lc code=end
