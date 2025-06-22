// Created by Ayush Biswas at 2025/06/19 10:50
// https://codeforces.com/problemset/problem/1525/B

use cp_lib::*;

// @code begin
use cpio::*;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> u8
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let a: Vec<usize> = input.read_line(parse_vec).unwrap();
    if a == (1..=n).collect::<Vec<_>>() {
        0
    } else if a[0] == 1 || a[n - 1] == n {
        1
    } else if a[0] != n || a[n - 1] != 1 {
        2
    } else {
        3
    }
}
// @code end
