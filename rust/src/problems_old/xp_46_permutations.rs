use core::num;

/*
 * @lc app=leetcode.cn id=46 lang=rust
 *
 * [46] Permutations
 *
 * https://leetcode.cn/problems/permutations/description/
 *
 * algorithms
 * Medium (80.17%)
 * Likes:    3361
 * Dislikes: 0
 * Total Accepted:    1.7M
 * Total Submissions: 2.2M
 * Testcase Example:  '[1,2,3]'
 *
 * Given an array nums of distinct integers, return all the possible
 * permutations. You can return the answer in any order.
 * 
 * 
 * Example 1:
 * Input: nums = [1,2,3]
 * Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
 * Example 2:
 * Input: nums = [0,1]
 * Output: [[0,1],[1,0]]
 * Example 3:
 * Input: nums = [1]
 * Output: [[1]]
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= nums.length <= 6
 * -10 <= nums[i] <= 10
 * All the integers of nums are unique.
 * 
 * 
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current = Vec::new();
        let mut used = vec![false; nums.len()];
        
        Self::back_track2(&nums, &mut current, &mut used, &mut result);
        
        result
    }

    fn back_track2(nums:&[i32],current:&mut Vec<i32>,used:&mut Vec<bool>,result:&mut Vec<Vec<i32>>){

        if current.len() ==nums.len()
        {
            result.push(current.clone());
            return;
        }

        for i in 0..nums.len(){
            if !used[i]{
                used[i] = true;
                current.push(nums[i]);
                
                Self::back_track2(nums, current, used, result);

                used[i] = false;
                current.pop();
            }
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    fn sort_result(mut result: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        result.sort();
        result
    }

    #[test]
    fn test_permute_example1() {
        let nums = vec![1, 2, 3];
        let mut result = Solution::permute(nums);
        result = sort_result(result);
        
        let expected = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_permute_example2() {
        let nums = vec![0, 1];
        let mut result = Solution::permute(nums);
        result = sort_result(result);
        
        let expected = vec![
            vec![0, 1],
            vec![1, 0],
        ];
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_permute_example3() {
        let nums = vec![1];
        let result = Solution::permute(nums);
        
        let expected = vec![vec![1]];
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_permute_empty() {
        let nums: Vec<i32> = vec![];
        let result = Solution::permute(nums);
        
        assert_eq!(result, vec![vec![]]);
    }
}