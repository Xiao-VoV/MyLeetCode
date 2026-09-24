use std::io::{self, *};

/// 检查 target 是否能由 n 个骰子拼成
/// cube_masks[d] = 包含数字 d 的骰子位掩码（低 n 位有效）
fn can_form(target: usize, cube_masks: &[u32; 10], n: usize) -> bool {
    let digits: Vec<u32> = target
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let k = digits.len();
    if k > n {
        return false;
    }

    // 预检：每个数字的出现次数不能超过拥有该数字的骰子数
    let mut need = [0u32; 10];
    for &d in &digits {
        need[d as usize] += 1;
    }
    for d in 0..10 {
        if need[d] > cube_masks[d].count_ones() {
            return false;
        }
    }

    // DFS 用位掩码做二分图匹配
    fn dfs(idx: usize, digits: &[u32], cube_masks: &[u32; 10], used: u32) -> bool {
        if idx == digits.len() {
            return true;
        }
        let d = digits[idx] as usize;
        let mut avail = cube_masks[d] & !used;
        while avail != 0 {
            let cube = avail.trailing_zeros();
            if dfs(idx + 1, digits, cube_masks, used | (1 << cube)) {
                return true;
            }
            avail &= avail - 1;
        }
        false
    }

    dfs(0, &digits, cube_masks, 0)
}

fn solve_case(n: usize, cube_masks: &[u32; 10]) -> usize {
    let mut target = 1;
    loop {
        if !can_form(target, cube_masks, n) {
            return target;
        }
        target += 1;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let count: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..count {
        let n: usize = iter.next().unwrap().parse().unwrap();

        // cube_masks[d] = 包含数字 d 的骰子位掩码
        let mut cube_masks = [0u32; 10];

        for i in 0..n {
            for _ in 0..6 {
                let num: usize = iter.next().unwrap().parse().unwrap();
                cube_masks[num] |= 1 << i;
            }
        }

        println!("{}", solve_case(n, &cube_masks));
    }
}