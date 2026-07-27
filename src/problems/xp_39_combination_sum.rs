use std::result;

/*
 * @lc app=leetcode.cn id=39 lang=rust
 *
 * [39] Combination Sum
 *
 * https://leetcode.cn/problems/combination-sum/description/
 *
 * algorithms
 * Medium (74.05%)
 * Likes:    3231
 * Dislikes: 0
 * Total Accepted:    1.5M
 * Total Submissions: 2.1M
 * Testcase Example:  '[2,3,6,7]\n7'
 *
 * Given an array of distinct integers candidates and a target integer target,
 * return a list of all unique combinations of candidates where the chosen
 * numbers sum to target. You may return the combinations in any order.
 * 
 * The same number may be chosen from candidates an unlimited number of times.
 * Two combinations are unique if the frequency of at least one of the chosen
 * numbers is different.
 * 
 * The test cases are generated such that the number of unique combinations
 * that sum up to target is less than 150 combinations for the given input.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: candidates = [2,3,6,7], target = 7
 * Output: [[2,2,3],[7]]
 * Explanation:
 * 2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple
 * times.
 * 7 is a candidate, and 7 = 7.
 * These are the only two combinations.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: candidates = [2,3,5], target = 8
 * Output: [[2,2,2,2],[2,3,3],[3,5]]
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: candidates = [2], target = 1
 * Output: []
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= candidates.length <= 30
 * 2 <= candidates[i] <= 40
 * All elements of candidates are distinct.
 * 1 <= target <= 40
 * 
 * 
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result  = Vec::new();
        let mut current = Vec::new();
        Self::back_tracking39(&candidates, &mut result, &mut current, target,0);
        result
    }

    pub fn back_tracking39(candidates:& Vec<i32>,result:&mut Vec<Vec<i32>>,current:&mut Vec<i32>,target: i32,start:usize){
        if current.iter().sum::<i32>() > target{
            return;
        }
        if current.iter().sum::<i32>() == target{
            result.push(current.clone());
            return;
        }

        for i in start..candidates.len(){
            current.push(candidates[i]);
            Self::back_tracking39(candidates, result, current, target-candidates[i],i);
            current.pop();
        }
    }
}
// @lc code=end

