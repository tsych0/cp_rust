// Created by Ayush Biswas at 2025/06/10 16:41
// https://codeforces.com/problemset/problem/1475/A

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
    let mut n: usize = input.read_line(parse).unwrap();
    while n % 2 == 0 {
        n /= 2
    }
    n != 1
}
// @code end
