// Created by Ayush Biswas at 2025/06/07 13:01
// https://codeforces.com/problemset/problem/151/A

use cp_lib::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let [n, k, l, c, d, p, nl, np]: [usize; 8] =
        input.read_line(parse_vec).unwrap().try_into().unwrap();
    ((k * l) / nl).min(c * d).min(p / np) / n
}
// @code end
