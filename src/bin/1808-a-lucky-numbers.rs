// Created by Ayush Biswas at 2025/06/13 10:07
// https://codeforces.com/problemset/problem/1808/A

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        [l, r]: [usize; 2]
    ) -> usize {
        if r - l < 100 {
            (l..=r).map(|i| (luckiness(i), i)).max().unwrap().1
        } else {
            (l..=r).find(|i| i % 100 == 90).unwrap()
        }
    }
}

fn luckiness(n: usize) -> u32 {
    let n = n.to_string();
    n.chars().max().unwrap().to_digit(10).unwrap() - n.chars().min().unwrap().to_digit(10).unwrap()
}

// @code end
