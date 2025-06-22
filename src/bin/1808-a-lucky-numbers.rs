// Created by Ayush Biswas at 2025/06/13 10:07
// https://codeforces.com/problemset/problem/1808/A

use cp_lib::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn luckiness(n: usize) -> u32 {
    let n = n.to_string();
    n.chars().max().unwrap().to_digit(10).unwrap() - n.chars().min().unwrap().to_digit(10).unwrap()
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let [l, r]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    if r - l < 100 {
        (l..=r).map(|i| (luckiness(i), i)).max().unwrap().1
    } else {
        (l..=r).find(|i| i % 100 == 90).unwrap()
    }
}
// @code end
