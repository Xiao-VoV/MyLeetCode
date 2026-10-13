/*
 * @lc app=leetcode.cn id=560 lang=rust
 *
 * [560] Subarray Sum Equals K
 *
 * https://leetcode.cn/problems/subarray-sum-equals-k/description/
 *
 * algorithms
 * Medium (46.96%)
 * Likes:    3250
 * Dislikes: 0
 * Total Accepted:    1.2M
 * Total Submissions: 2.6M
 * Testcase Example:  '[1,1,1]\n2'
 *
 * Given an array of integers nums and an integer k, return the total number of
 * subarrays whose sum equals to k.
 *
 * A subarray is a contiguous non-empty sequence of elements within an
 * array.
 *
 *
 * Example 1:
 * Input: nums = [1,1,1], k = 2
 * Output: 2
 * Example 2:
 * Input: nums = [1,2,3], k = 3
 * Output: 2
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 2 * 10^4
 * -1000 <= nums[i] <= 1000
 * -10^7 <= k <= 10^7
 *
 *
 */
use super::Solution;

// @lc code=start
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();

        let mut prefix = vec![0; n + 1];
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(0, 1);
        let mut result = 0;
        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i];
            result += map.get(&(prefix[i + 1] - k)).unwrap_or(&0);
            *map.entry(prefix[i + 1]).or_insert(0) += 1;
        }
        result
    }
}
// @lc code=end
