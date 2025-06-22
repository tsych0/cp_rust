// Created by Ayush Biswas at 2025/06/14 13:48
// https://codeforces.com/problemset/problem/1883/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::collections::HashMap;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> bool
where
    R: Read,
{
    let [_n, k]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let s: String = input.read_line(parse).unwrap();
    let count: HashMap<char, usize> = s
        .chars()
        .sorted()
        .group_by(|&x| x)
        .map(|g| (g[0], g.len()))
        .collect();
    count.values().filter(|&&v| v % 2 == 1).count() <= k + 1
}
// @code end
