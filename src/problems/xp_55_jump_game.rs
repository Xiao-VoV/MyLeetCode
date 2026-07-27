/*
 * @lc app=leetcode.cn id=55 lang=rust
 *
 * [55] Jump Game
 *
 * https://leetcode.cn/problems/jump-game/description/
 *
 * algorithms
 * Medium (45.16%)
 * Likes:    3292
 * Dislikes: 0
 * Total Accepted:    1.6M
 * Total Submissions: 3.5M
 * Testcase Example:  '[2,3,1,1,4]'
 *
 * You are given an integer array nums. You are initially positioned at the
 * array's first index, and each element in the array represents your maximum
 * jump length at that position.
 * 
 * Return true if you can reach the last index, or false otherwise.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: nums = [2,3,1,1,4]
 * Output: true
 * Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last
 * index.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: nums = [3,2,1,0,4]
 * Output: false
 * Explanation: You will always arrive at index 3 no matter what. Its maximum
 * jump length is 0, which makes it impossible to reach the last index.
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= nums.length <= 10^4
 * 0 <= nums[i] <= 10^5
 * 
 * 
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut record = vec![-1;nums.len()];

        for (k,v) in nums.iter().enumerate(){
            record[k] = k as i32 + *v;
        }
        let mut pre_max = 0;
        for (k,v) in record.iter().enumerate(){
            if pre_max < k as i32{
                return false;
            }
            pre_max = pre_max.max(*v);
        }
        true
    }
}
// @lc code=end

