/*
 * @lc app=leetcode.cn id=41 lang=rust
 *
 * [41] First Missing Positive
 *
 * https://leetcode.cn/problems/first-missing-positive/description/
 *
 * algorithms
 * Hard (49.33%)
 * Likes:    2568
 * Dislikes: 0
 * Total Accepted:    763.4K
 * Total Submissions: 1.5M
 * Testcase Example:  '[1,2,0]'
 *
 * Given an unsorted integer array nums. Return the smallest positive integer
 * that is not present in nums.
 *
 * You must implement an algorithm that runs in O(n) time and uses O(1)
 * auxiliary space.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,2,0]
 * Output: 3
 * Explanation: The numbers in the range [1,2] are all in the array.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [3,4,-1,1]
 * Output: 2
 * Explanation: 1 is in the array but 2 is missing.
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [7,8,9,11,12]
 * Output: 1
 * Explanation: The smallest positive integer 1 is missing.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^5
 * -2^31 <= nums[i] <= 2^31 - 1
 *
 *
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();

        for i in 0..len {
            while nums[i] > 0 && nums[i] <= len as i32 && nums[i] != nums[(nums[i] - 1) as usize] {
                let b = (nums[i] - 1) as usize;
                nums.swap(i, b);
            }
        }

        for i in 0..len {
            if nums[i] != (i + 1) as i32 {
                return (i + 1) as i32;
            }
        }

        return len as i32 + 1;
    }
}
// @lc code=end
