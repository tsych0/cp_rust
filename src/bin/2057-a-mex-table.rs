// Created by Ayush Biswas at 2025/05/14 18:17
// https://codeforces.com/problemset/problem/2057/A
use cp_rust::*;

// @code begin
use cpio::solve_n;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut cpio::CPInput<R>) -> usize
where
    R: Read,
{
    let [n, m]: [usize; 2] = input
        .read_line(cpio::parse_vec)
        .unwrap()
        .try_into()
        .unwrap();
    usize::max(n, m) + 1
}
// @code end
