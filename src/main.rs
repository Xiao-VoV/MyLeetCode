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
//解题思路
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

fn main() {
    println!("Hello, world!");
}
