/*
 * @lc app=leetcode.cn id=239 lang=rust
 *
 * [239] Sliding Window Maximum
 *
 * https://leetcode.cn/problems/sliding-window-maximum/description/
 *
 * algorithms
 * Hard (50.20%)
 * Likes:    3477
 * Dislikes: 0
 * Total Accepted:    1.2M
 * Total Submissions: 2.3M
 * Testcase Example:  '[1,3,-1,-3,5,3,6,7]\n3'
 *
 * You are given an array of integers nums, there is a sliding window of size k
 * which is moving from the very left of the array to the very right. You can
 * only see the k numbers in the window. Each time the sliding window moves
 * right by one position.
 *
 * Return the max sliding window.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,3,-1,-3,5,3,6,7], k = 3
 * Output: [3,3,5,5,6,7]
 * Explanation:
 * Window position                Max
 * ---------------               -----
 * [1  3  -1] -3  5  3  6  7       3
 * ⁠1 [3  -1  -3] 5  3  6  7       3
 * ⁠1  3 [-1  -3  5] 3  6  7       5
 * ⁠1  3  -1 [-3  5  3] 6  7       5
 * ⁠1  3  -1  -3 [5  3  6] 7       6
 * ⁠1  3  -1  -3  5 [3  6  7]      7
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [1], k = 1
 * Output: [1]
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^5
 * -10^4 <= nums[i] <= 10^4
 * 1 <= k <= nums.length
 *
 *
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        // for i in k..=nums.len() as i32 {
        //     let start = (i - k) as usize;
        //     let end = start + k as usize;
        //     let slice = &nums[start..end].iter().max().unwrap();
        //     result.push(**slice);
        // }
        use std::collections::VecDeque;
        let mut deque = VecDeque::<(i32, usize)>::new();
        let k = k as usize;

        for (i, v) in nums.iter().enumerate() {
            if let Some(&(_, index)) = deque.front() {
                if index + k <= i {
                    deque.pop_front();
                }
            }

            while let Some(&(value, _)) = deque.back() {
                if value < *v {
                    deque.pop_back();
                } else {
                    break;
                }
            }

            deque.push_back((*v, i));

            if i + 1 >= k {
                result.push(deque.front().unwrap().0);
            }
        }
        result
    }
}
// @lc code=end
