// Created by Ayush Biswas at 2025/06/12 15:23
// https://codeforces.com/problemset/problem/1954/A

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
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
    let [n, m, k]: [isize; 3] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    k < (n - (n + m - 1) / m)
}
// @code end
