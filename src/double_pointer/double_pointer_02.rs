//leetcode 2824
pub fn count_pairs(mut nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 1 {
        return 0;
    }
    nums.sort();
    let mut left: usize = 0;
    let mut right: usize = nums.len() - 1;
    let mut count: usize = 0;
    while left < right {
        if nums[left] + nums[right] < target {
            count += right - left;
            left += 1;
        } else {
            right -= 1;
        }
    }
    count as i32
}

//letcode 2958
use std::{
    collections::{hash_map, HashMap},
    hash::Hash,
};

pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut show_count = HashMap::new();
    for i in nums.clone() {
        *show_count.entry(i).or_insert(0) += 1;
    }
    let mut left: i32 = 0;
    let mut right: i32 = nums.len() as i32 - 1;
    let mut length = 0;
    while left <= right {
        while *show_count.get(&nums[right as usize]).unwrap() > k {
            right -= 1;
        }
        while *show_count.get(&nums[left as usize]).unwrap() > k {
            left += 1;
        }
        length = right - left + 1;
    }
    length
}

//leetcode16
pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort_unstable();
    let length = nums.len() - 1;
    let mut result = 0;
    let mut inaccuracies = i32::MAX;
    for i in (0..nums.len() - 2) {
        let x = nums[i];
        let mut left = i + 1;
        let mut right = length;
        while left < right {
            let sum = nums[left] + nums[right] + x;
            if (target - sum).abs() < inaccuracies {
                result = sum;
                inaccuracies = (target - sum).abs();
            }
            if sum > target {
                right -= 1;
            } else if sum < target {
                left += 1;
            } else {
                return target;
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_leetcode2958() {
        max_subarray_length(vec![1, 2, 3, 1, 2, 3, 1, 2], 2);
    }
    #[test]
    pub fn test_leetcode_2824() {
        println!("{}", count_pairs(vec![-6, 2, 5, -2, -7, -1, 3], -2));
    }
    #[test]
    pub fn test_leetcode_16() {
        let x = three_sum_closest(vec![-1, 2, 1, -4], 1);
        println!("{x}");
    }
}
