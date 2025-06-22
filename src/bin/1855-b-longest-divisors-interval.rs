// Created by Ayush Biswas at 2025/06/12 23:17
// https://codeforces.com/problemset/problem/1855/B

use cp_lib::*;

// @code begin
use cpio::*;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    (1..).take_while(|i| n % i == 0).count()
}
// @code end
