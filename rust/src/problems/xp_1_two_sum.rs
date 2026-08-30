/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] Two Sum
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // for i in 0..nums.len() {
        //     for j in i + 1..nums.len() {
        //         if nums[i] + nums[j] == target {
        //             return vec![i as i32, j as i32];
        //         }
        //     }
        // }
        // vec![]

        use std::collections::HashMap;
        let mut map: HashMap<i32, usize> = HashMap::new();
        for (k, v) in nums.iter().enumerate() {
            let need = target - v;
            if map.contains_key(&need) {
                return vec![k as i32, map[&need] as i32];
            } else {
                map.insert(*v, k);
            }
        }
        vec![]
    }
}
// @lc code=end
