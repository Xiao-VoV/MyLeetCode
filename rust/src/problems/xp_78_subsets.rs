/*
 * @lc app=leetcode.cn id=78 lang=rust
 *
 * [78] Subsets
 *
 * https://leetcode.cn/problems/subsets/description/
 *
 * algorithms
 * Medium (81.85%)
 * Likes:    2665
 * Dislikes: 0
 * Total Accepted:    1.3M
 * Total Submissions: 1.6M
 * Testcase Example:  '[1,2,3]'
 *
 * Given an integer array nums of unique elements, return all possible subsets
 * (the power set).
 *
 * The solution set must not contain duplicate subsets. Return the solution in
 * any order.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,2,3]
 * Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [0]
 * Output: [[],[0]]
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10
 * -10 <= nums[i] <= 10
 * All the numbers of nums are unique.
 *
 *
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current = Vec::new();

        Self::back_track78(&nums, &mut result, 0, &mut current);

        result
    }

    fn back_track78(
        nums: &[i32],
        result: &mut Vec<Vec<i32>>,
        start: usize,
        current: &mut Vec<i32>,
    ) {
        if current.len() == nums.len(){
            return;
        }
        result.push(current.clone());

        for i in 0..nums.len() {
            current.push(nums[i]);

            Self::back_track78(nums, result, i + 1, current);
            current.pop();
        }
    }
}
// @lc code=end
