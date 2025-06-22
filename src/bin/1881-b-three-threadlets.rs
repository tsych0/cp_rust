// Created by Ayush Biswas at 2025/06/21 13:56
// https://codeforces.com/problemset/problem/1881/B

use cp_lib::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> bool
where
    R: Read,
{
    let [a, b, c]: [usize; 3] = input.read_line(parse_vec).unwrap().try_into().unwrap();

    false
}
// @code end
