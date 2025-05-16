// Created by Ayush Biswas at 2025/05/14 20:46
// https://codeforces.com/problemset/problem/50/A
use cf_rust::cpio;

// @code begin
use cpio::solve;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut cpio::CPInput<R>) -> u16
where
    R: Read,
{
    let [m, n]: [u16; 2] = input
        .read_line(cpio::parse_vec)
        .unwrap()
        .try_into()
        .unwrap();
    (m * n) / 2
}
// @code end
