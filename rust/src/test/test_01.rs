use std::{
    collections::HashSet,
    io::{self, *},
};

fn form_number(target: usize, cubes: &Vec<HashSet<usize>>) -> bool {
    let digts: Vec<usize> = target
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    if digts.len() > cubes.len() {
        return false;
    }

    let mut used = vec![false; cubes.len()];

    fn dfs(
        index: usize,
        digts: &Vec<usize>,
        cubes: &Vec<HashSet<usize>>,
        used: &mut Vec<bool>,
    ) -> bool {
        if index == digts.len() {
            return false;
        }
        let d = digts[index];

        for i in 0..cubes.len() {
            if !used[i] && cubes[i].contains(&d) {
                used[i] = true;
                if dfs(index + 1, digts, cubes, used) {
                    return true;
                }

                used[i] = false;
            }
        }

        false
    }

    dfs(0, &digts, cubes, &mut used)
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let count: usize = iter.next().unwrap().parse().unwrap();

    for i in 0..count {
        let n = iter.next().unwrap().parse().unwrap();

        let mut cubes = Vec::with_capacity(n);

        for _ in 0..n {
            let mut set = HashSet::new();

            for _ in 0..6 {
                let num = iter.next().unwrap().parse::<usize>().unwrap();
                set.insert(num);
            }
            cubes.push(set);
        }

        let mut target = 1;

        loop {
            if !form_number(target, &cubes) {
                println!("{}", target);
                break;
            }
        }
        target +=1;
    }
}
