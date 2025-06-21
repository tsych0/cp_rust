// Created by Ayush Biswas at 2025/05/14 10:38
// https://codeforces.com/problemset/problem/2067/A
use cp_rust::*;

// @code begin
use cpio::solve_n;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut cpio::CPInput<R>) -> bool
where
    R: Read,
{
    let [x, y]: [usize; 2] = input
        .read_line(cpio::parse_vec)
        .unwrap()
        .try_into()
        .unwrap();
    if x > y {
        (x - y + 1) % 9 == 0
    } else {
        (y - x) == 1
    }
}
// @code end
