// Created by Ayush Biswas at 2025/05/17 14:18
// https://codeforces.com/problemset/problem/2013/A
use cp_rust::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let [x, y]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    n.div_ceil(x.min(y))
}
// @code end
