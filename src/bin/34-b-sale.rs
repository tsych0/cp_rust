// Created by Ayush Biswas at 2025/06/12 16:20
// https://codeforces.com/problemset/problem/34/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> isize
where
    R: Read,
{
    let [n, m]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let a: Vec<isize> = input.read_line(parse_vec).unwrap();
    -a.into_iter()
        .filter(|&i| i < 0)
        .sorted()
        .take(m)
        .sum::<isize>()
}
// @code end
