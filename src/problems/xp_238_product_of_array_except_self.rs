/*
 * @lc app=leetcode.cn id=238 lang=rust
 *
 * [238] Product of Array Except Self
 *
 * https://leetcode.cn/problems/product-of-array-except-self/description/
 *
 * algorithms
 * Medium (77.79%)
 * Likes:    2254
 * Dislikes: 0
 * Total Accepted:    997.5K
 * Total Submissions: 1.3M
 * Testcase Example:  '[1,2,3,4]'
 *
 * Given an integer array nums, return an array answer such that answer[i] is
 * equal to the product of all the elements of nums except nums[i].
 *
 * The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit
 * integer.
 *
 * You must write an algorithm that runs in O(n) time and without using the
 * division operation.
 *
 *
 * Example 1:
 * Input: nums = [1,2,3,4]
 * Output: [24,12,8,6]
 * Example 2:
 * Input: nums = [-1,1,0,-3,3]
 * Output: [0,0,9,0,0]
 *
 *
 * Constraints:
 *
 *
 * 2 <= nums.length <= 10^5
 * -30 <= nums[i] <= 30
 * The input is generated such that answer[i] is guaranteed to fit in a 32-bit
 * integer.
 *
 *
 *
 * Follow up: Can you solve the problem in O(1) extra space complexity? (The
 * output array does not count as extra space for space complexity analysis.)
 *
 */

use super::Solution;
// @lc code=start
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut fore = vec![1;len];
        let mut back = vec![1;len];

        for i in 1..len{
            fore[i] = fore[i-1] * nums[i-1];
        }


        for i in (0..len - 1).rev() {
            back[i] = back[i + 1] * nums[i + 1];
        }

        let mut result = Vec::with_capacity(len);
        for i in 0..len {
            result.push(fore[i] * back[i]);
        }
        result
    }
}
// @lc code=end
