/**leetcod 3
 *
*/
use std::{collections::HashMap, ops::Sub};
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut left = 0;
    let mut length = 0;
    let mut hash = HashMap::<char, i32>::new();
    for (right, value) in s.chars().into_iter().enumerate() {
        *hash.entry(value).or_insert(0) += 1;
        while *hash.get(&value).unwrap() > 1 {
            let char_in_s = s.chars().nth(left).unwrap();
            *hash.entry(char_in_s).or_default() -= 1;
            left += 1;
        }
        length = length.max((right - left + 1) as i32);
    }
    length
}

//leetcode34
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() == 0 {
        return vec![-1, -1];
    }
    let mut left: i32 = 0;
    let mut right: i32 = (nums.len() - 1) as i32;
    let mut mid: i32 = 0;
    while left <= right {
        mid = left + (right - left) / 2;
        if nums[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    if left > nums.len() as i32 -1 || nums[left as usize] != target {
        return vec![-1, -1];
    }
    right  = left;
    while left >=0 && nums[left as usize] == target {
        left -= 1;
    }
    while right <= nums.len() as i32 -1 && nums[right as usize] == target {
        right += 1;
    }
    vec![left +1 as i32, right -1 as i32]
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn leedcode_3_test() {
        let s = String::from("abcabcbb");
        println!("{}", length_of_longest_substring(s));
    }
    #[test]
    pub fn leetcode_34_test() {
        // assert_eq!(search_range(vec![5,7,7,8,8,10], 8),[3,4]);
        // assert_eq!(search_range(vec![1], 1), [0,0]);
        assert_eq!(search_range(vec![2,2], 3), [-1,-1]);
    }
}
