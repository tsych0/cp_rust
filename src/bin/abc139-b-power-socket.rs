// Created by Ayush Biswas at 2025/06/23 16:34
// https://atcoder.jp/contests/abc139/tasks/abc139_b
use cp_lib::*;

// @code begin
use cpio::*;
#[allow(unused)]
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let [a, b]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    1 + (b - 2) / (a - 1)
}
// @code end
