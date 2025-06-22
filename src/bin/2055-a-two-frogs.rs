// Created by Ayush Biswas at 2025/05/14 20:41
// https://codeforces.com/problemset/problem/2055/A
use cp_lib::*;

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
    let [_n, a, b]: [i8; 3] = input
        .read_line(cpio::parse_vec)
        .unwrap()
        .try_into()
        .unwrap();
    (a - b).abs() % 2 == 0
}
// @code end
