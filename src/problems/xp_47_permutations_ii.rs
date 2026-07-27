/*
 * @lc app=leetcode.cn id=47 lang=rust
 *
 * [47] Permutations II
 *
 * https://leetcode.cn/problems/permutations-ii/description/
 *
 * algorithms
 * Medium (66.78%)
 * Likes:    1814
 * Dislikes: 0
 * Total Accepted:    738.2K
 * Total Submissions: 1.1M
 * Testcase Example:  '[1,1,2]'
 *
 * Given a collection of numbers, nums, that might contain duplicates, return
 * all possible unique permutations in any order.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,1,2]
 * Output:
 * [[1,1,2],
 * ⁠[1,2,1],
 * ⁠[2,1,1]]
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [1,2,3]
 * Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 8
 * -10 <= nums[i] <= 10
 *
 *
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current = Vec::new();
        let mut used = vec![false; nums.len()];
        nums.sort();
        Self::back_track(&nums, &mut current, &mut used, &mut result);

        result
    }

    fn back_track(
        nums: &[i32],
        current: &mut Vec<i32>,
        used: &mut Vec<bool>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if current.len() == nums.len() {
            result.push(current.clone());
            return;
        }

        for i in 0..nums.len() {
            if used[i] {
                continue;
            }

            if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
                continue;
            }

            used[i] = true;
            current.push(nums[i]);

            Self::back_track(nums, current, used, result);

            used[i] = false;
            current.pop();
        }
    }
}
// @lc code=end
