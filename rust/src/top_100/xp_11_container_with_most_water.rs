/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] Container With Most Water
 *
 * https://leetcode.cn/problems/container-with-most-water/description/
 *
 * algorithms
 * Medium (61.90%)
 * Likes:    6068
 * Dislikes: 0
 * Total Accepted:    2.3M
 * Total Submissions: 3.8M
 * Testcase Example:  '[1,8,6,2,5,4,8,3,7]'
 *
 * You are given an integer array height of length n. There are n vertical
 * lines drawn such that the two endpoints of the i^th line are (i, 0) and (i,
 * height[i]).
 *
 * Find two lines that together with the x-axis form a container, such that the
 * container contains the most water.
 *
 * Return the maximum amount of water a container can store.
 *
 * Notice that you may not slant the container.
 *
 *
 * Example 1:
 *
 *
 * Input: height = [1,8,6,2,5,4,8,3,7]
 * Output: 49
 * Explanation: The above vertical lines are represented by array
 * [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the
 * container can contain is 49.
 *
 *
 * Example 2:
 *
 *
 * Input: height = [1,1]
 * Output: 1
 *
 *
 *
 * Constraints:
 *
 *
 * n == height.length
 * 2 <= n <= 10^5
 * 0 <= height[i] <= 10^4
 *
 *
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0usize;
        let mut right = height.len()-1;

        let mut max_comtain = 0i32;

        while left < right {
            let mut heig = height[left].min(height[right]);

            max_comtain = max_comtain.max((right - left) as i32 * heig);

            if height[left] > height[right]{
                right -=1;
            }else{
                left +=1;
            }
        }

        max_comtain
    }
}
// @lc code=end
