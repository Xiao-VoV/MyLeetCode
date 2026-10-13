use std::iter::StepBy;

/*
 * @lc app=leetcode.cn id=56 lang=rust
 *
 * [56] Merge Intervals
 *
 * https://leetcode.cn/problems/merge-intervals/description/
 *
 * algorithms
 * Medium (54.25%)
 * Likes:    2876
 * Dislikes: 0
 * Total Accepted:    1.7M
 * Total Submissions: 3.1M
 * Testcase Example:  '[[1,3],[2,6],[8,10],[15,18]]'
 *
 * Given an array of intervals where intervals[i] = [starti, endi], merge all
 * overlapping intervals, and return an array of the non-overlapping intervals
 * that cover all the intervals in the input.
 *
 *
 * Example 1:
 *
 *
 * Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
 * Output: [[1,6],[8,10],[15,18]]
 * Explanation: Since intervals [1,3] and [2,6] overlap, merge them into
 * [1,6].
 *
 *
 * Example 2:
 *
 *
 * Input: intervals = [[1,4],[4,5]]
 * Output: [[1,5]]
 * Explanation: Intervals [1,4] and [4,5] are considered overlapping.
 *
 *
 * Example 3:
 *
 *
 * Input: intervals = [[4,7],[1,4]]
 * Output: [[1,7]]
 * Explanation: Intervals [1,4] and [4,7] are considered overlapping.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= intervals.length <= 10^4
 * intervals[i].length == 2
 * 0 <= starti <= endi <= 10^4
 *
 *
 */
use crate::Solution;
// @lc code=start
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort();

        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut start = intervals[0][0];
        let mut end = intervals[0][1];

        for i in 1..intervals.len() {
            if intervals[i][0] <= end || intervals[i][1] <= end{
                end = intervals[i][1].max(end);
            } else {
                result.push(vec![start, end]);
                start = intervals[i][0];
                end = intervals[i][1];
            }
        }
        result.push(vec![start, end]);

        result
    }
}
// @lc code=end
