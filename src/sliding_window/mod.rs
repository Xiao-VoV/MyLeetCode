struct Solution {}
/**
 * leetcode 2958
 */
use std::collections::HashMap;
pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    if nums.len() == 1 && k >= 1 {
        return 1;
    }
    let mut left = 0;
    let mut count_map = HashMap::<i32, usize>::new();
    let mut result = 0;
    for (index, value) in nums.iter().enumerate() {
        *count_map.entry(*value).or_insert(0) += 1;
        if *count_map.get(value).unwrap() <= k as usize {
            result = result.max(index - left + 1);
        }
        while *count_map.get(value).unwrap() > k as usize {
            *count_map.entry(nums[left]).or_insert(0) -= 1;
            left += 1;
        }
    }
    result as i32
}

/**
 * leetcode 2730
 */
impl Solution {
    pub fn longest_semi_repetitive_substring(s: String) -> i32 {
        let mut left = 0;
        let mut result = 0;
        let chars: Vec<char> = s.chars().collect::<Vec<char>>();
        let mut first_pair = 0;
        for (index, value) in chars.iter().enumerate() {
            let left_ = left;
            if index >= 2 && *value == chars[index - 1] {
                loop {
                    left += 1;
                    if chars[left] == chars[left - 1] {
                        if left == index {
                            left = left_;
                        }
                        break;
                    }
                }
            }
            result = result.max(index - left + 1);
        }
        result as i32
    }
}
//leetcode 209
//滑动窗口
/**
 * 这个是滑动窗口的标准写法，可以背下来
 */
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_leetcode_1958() {
        // let x = max_subarray_length(vec![1, 2, 3, 1, 2, 3, 1, 2], 2);
        // print!("{x}");
        // let y = max_subarray_length(vec![1], 1);
        // print!("{y}");
        // let y = max_subarray_length(vec![1, 11], 2);
        // print!("{y}");
        let y = max_subarray_length(vec![1, 4, 4, 3], 1);
        print!("{y}");
    }
    #[test]
    pub fn test_leetcode_2730() {
        let x = Solution::longest_semi_repetitive_substring("52233".to_string());
        print!("{x}")
    }
}
