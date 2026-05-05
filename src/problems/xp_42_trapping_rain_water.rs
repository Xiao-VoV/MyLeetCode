use std::vec;

/*
 * @lc app=leetcode.cn id=42 lang=rust
 *
 * [42] Trapping Rain Water
 *
 * https://leetcode.cn/problems/trapping-rain-water/description/
 *
 * algorithms
 * Hard (66.01%)
 * Likes:    6276
 * Dislikes: 0
 * Total Accepted:    1.8M
 * Total Submissions: 2.7M
 * Testcase Example:  '[0,1,0,2,1,0,1,3,2,1,2,1]'
 *
 * Given n non-negative integers representing an elevation map where the width
 * of each bar is 1, compute how much water it can trap after raining.
 *
 *
 * Example 1:
 *
 *
 * Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
 * Output: 6
 * Explanation: The above elevation map (black section) is represented by array
 * [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue
 * section) are being trapped.
 *
 *
 * Example 2:
 *
 *
 * Input: height = [4,2,0,3,2,5]
 * Output: 9
 *
 *
 *
 * Constraints:
 *
 *
 * n == height.length
 * 1 <= n <= 2 * 10^4
 * 0 <= height[i] <= 10^5
 *
 *
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut left_max = vec![0; n];
        let mut right_max = vec![0; n];

        let mut max = 0;
        for i in 0..n {
            max = max.max(height[i]);
            left_max[i] = max;
        }
        max = 0;
        for i in (0..n).rev() {
            max = max.max(height[i]);
            right_max[i] = max;
        }

        let mut result = 0;

        for i in 0..n {
            result += left_max[i].min(right_max[i]) - height[i];
        }

        result
    }
}
// @lc code=end
