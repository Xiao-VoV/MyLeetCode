use core::num;

/***
 *2807. 在链表中插入最大公约数
 *中等
 *相关标签
 *相关企业
 *给你一个链表的头 head ，每个结点包含一个整数值。
 *在相邻结点之间，请你插入一个新的结点，结点值为这两个相邻结点值的 最大公约数 。
 *请你返回插入之后的链表。
 *两个数的 最大公约数 是可以被两个数字整除的最大正整数。
 *
 *
 */
mod leetcode_2807 {
    use std::borrow::Borrow;

    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }
    pub fn insert_greatest_common_divisors(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut current = &mut head;
        while current.as_ref().unwrap().next.is_some() {
            let x = current.as_mut().unwrap();
            let next = x.next.take();
            x.next = Some(Box::new(ListNode {
                val: get_common_divisors(x.val, next.as_ref().unwrap().val),
                next,
            }));
            current = &mut current.as_mut().unwrap().next.as_mut().unwrap().next;
        }
        None
    }
    fn get_common_divisors(x: i32, y: i32) -> i32 {
        let min = x.min(y);
        for i in (1..=min).rev() {
            if y % min == 0 {
                return i;
            }
        }
        1
    }
}
//leetcode 167. 两数之和 II - 输入有序数组
//解题思路 快排变种类型
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left: usize = 0;
    let mut right: usize = numbers.len() - 1;
    while (numbers[left] + numbers[right]) != target {
        while left < right && (numbers[left] + numbers[right]) > target {
            right -= 1;
        }
        while left < right && (numbers[left] + numbers[right]) < target {
            left += 1;
        }
    }
    println!("{} {}", numbers[left], numbers[right]);
    vec![(left + 1) as i32, (right + 1) as i32]
}

//leet code 25
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut result: Vec<Vec<i32>> = vec![vec![]];
    for i in 0..=nums.len() - 3 {
        let mut left: usize = 0;
        let mut right: usize = nums.len() - 1;
        while left < right && (nums[left] + nums[right]) > -nums[i] || right == i {
            right -= 1;
        }
        while left < right && (nums[left] + nums[right]) < nums[i] || left == i {
            left += 1;
        }
        result.push(vec![left as i32, right as i32, i as i32])
    }
    result
}
#[test]
pub fn leetcode_25_test() {
    let v = vec![-1, 0, 1, 2, -1, -4];
    let x = three_sum(v);
    print!("{:?}", x);
}

#[test]
pub fn leetcode_167_test() {
    let x = two_sum(
        vec![
            12, 13, 23, 28, 43, 44, 59, 60, 61, 68, 70, 86, 88, 92, 124, 125, 136, 168, 173, 173,
            180, 199, 212, 221, 227, 230, 277, 282, 306, 314, 316, 321, 325, 328, 336, 337, 363,
            365, 368, 370, 370, 371, 375, 384, 387, 394, 400, 404, 414, 422, 422, 427, 430, 435,
            457, 493, 506, 527, 531, 538, 541, 546, 568, 583, 585, 587, 650, 652, 677, 691, 730,
            737, 740, 751, 755, 764, 778, 783, 785, 789, 794, 803, 809, 815, 847, 858, 863, 863,
            874, 887, 896, 916, 920, 926, 927, 930, 933, 957, 981, 997,
        ],
        542,
    );
    print!("{:?}", x);
}

/**
 * leetocde 11
 *
 */
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left: usize = 0;
    let mut right: usize = height.len() - 1;
    let mut area = height[left].min(height[right]) * (right - left) as i32;
    while left < right {
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
        area = area.max(height[left].min(height[right]) * (right - left) as i32);
    }
    area
}

/**
* leetcode 42
*/
pub fn trap(height: Vec<i32>) -> i32 {
    let mut prefix_max_vec: Vec<i32> = Vec::new();
    let mut suffix_max_vec: Vec<i32> = vec![-1; height.len()];
    let mut x = -1;
    for i in height.iter() {
        x = x.max(*i);
        prefix_max_vec.push(x);
    }
    x = -1;
    let length = height.len() - 1;
    for (i, v) in height.iter().rev().enumerate() {
        x = x.max(*v);
        suffix_max_vec[length - i] = x;
    }
    let mut result = 0;
    for i in 0..height.len() {
        result += std::cmp::min(prefix_max_vec[i], suffix_max_vec[i]) - height[i];
    }
    result
}

#[test]
pub fn leetcode_42_test() {
    let vec = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    println!("leetcode 42 : {}", trap(vec));
}

fn main() {
    println!("Hello, world!");
}
