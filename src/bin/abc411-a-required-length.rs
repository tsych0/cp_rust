// Created by Ayush Biswas at 2025/06/21 17:35
// https://atcoder.jp/contests/abc411/tasks/abc411_a

use cp_lib::*;

// @code begin
use cpio::*;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> bool
where
    R: Read,
{
    let s: String = input.read_line(parse).unwrap();
    let n: usize = input.read_line(parse).unwrap();
    s.len() >= n
}
// @code end
