// Created by Ayush Biswas at 2025/06/12 13:14
// https://codeforces.com/problemset/problem/1992/C

use cp_lib::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> Words<usize>
where
    R: Read,
{
    let [n, m, _k]: [usize; 3] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    vec![
        (m + 1..=n).rev().collect::<Vec<_>>(),
        (1..=m).collect::<Vec<_>>(),
    ]
    .concat()
    .into()
}
// @code end
