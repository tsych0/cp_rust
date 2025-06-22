// Created by Ayush Biswas at 2025/06/21 11:30
// https://codeforces.com/contest/2121/problem/A

use cp_lib::*;

// @code begin
use cpio::*;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> bool
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    n > 2 && n % 2 == 0
}
// @code end
