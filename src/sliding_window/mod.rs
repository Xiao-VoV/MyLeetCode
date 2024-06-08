use std::collections::btree_map::Values;

pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    0
}
//leetcode 209
//滑动窗口
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut left = 0;
    // for (value, index) in nums.iter().enumerate() {

    // }
    let mut count: i32 = 0;
    let mut min_len: usize = nums.len() + 1;
    for (index, value) in nums.iter().enumerate() {
        count += value;
        while count >= target {
            count -= nums[left];
            left += 1;
        }
        min_len = min_len.min(index - left + 1);
    }
    min_len as i32
}
