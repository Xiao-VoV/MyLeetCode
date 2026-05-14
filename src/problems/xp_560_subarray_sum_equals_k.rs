use std::collections::HashMap;

/*
 * @lc app=leetcode.cn id=560 lang=rust
 *
 * [560] Subarray Sum Equals K
 *
 * https://leetcode.cn/problems/subarray-sum-equals-k/description/
 *
 * algorithms
 * Medium (46.50%)
 * Likes:    3154
 * Dislikes: 0
 * Total Accepted:    1.1M
 * Total Submissions: 2.3M
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
        use std::collections::HashMap;
        let mut prefix_sum = 0;
        let mut hash = HashMap::new();
        hash.entry(prefix_sum).or_insert(1);
        let mut result = 0;
        for i in nums {
            prefix_sum += i;
            result += hash.get(&(prefix_sum - k)).cloned().unwrap_or(0);
            hash.entry(prefix_sum).and_modify(|v| *v += 1).or_insert(1);
        }
        result
    }
}
// @lc code=end
