

/*
 * @lc app=leetcode.cn id=239 lang=rust
 *
 * [239] Sliding Window Maximum
 *
 * https://leetcode.cn/problems/sliding-window-maximum/description/
 *
 * algorithms
 * Hard (50.38%)
 * Likes:    3579
 * Dislikes: 0
 * Total Accepted:    1.3M
 * Total Submissions: 2.6M
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
use crate::Solution;
// @lc code=start
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();

        let mut result = Vec::with_capacity(n-k+1);


        use std::{collections::BinaryHeap};
        let mut heap = BinaryHeap::new();

        for i in 0..n{
            heap.push((nums[i],i));

            while heap.peek().unwrap().1 +k <=i {
                heap.pop();
            }
            if i >= k-1{
                result.push(heap.peek().unwrap().0);
            }
        }
        result
    }
}
// @lc code=end
