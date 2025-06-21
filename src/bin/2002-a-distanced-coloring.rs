// Created by Ayush Biswas at 2025/05/18 20:16
// https://codeforces.com/problemset/problem/2002/A
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
    let [n, m, k]: [usize; 3] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    n.min(k) * m.min(k)
}
// @code end
