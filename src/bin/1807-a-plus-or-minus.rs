// Created by Ayush Biswas at 2025/06/03 20:01
// https://codeforces.com/problemset/problem/1807/A

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> char
where
    R: Read,
{
    let [a, b, c]: [isize; 3] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    if a + b == c {
        '+'
    } else {
        '-'
    }
}
// @code end
