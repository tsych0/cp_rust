// Created by Ayush Biswas at 2025/06/14 16:01
// https://codeforces.com/problemset/problem/1733/B
#![allow(unused)]

use cp_rust::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> Words<isize>
where
    R: Read,
{
    let [n, x, y]: [isize; 3] = input.read_line(parse_vec).unwrap().try_into().unwrap();

    let k = x + y;
    if x != 0 && y != 0 || x + y == 0 || ((n - 1) % k != 0) {
        ListOf(vec![-1])
    } else {
        ListOf(
            vec![
                vec![1; k as usize],
                (k..n - 1).map(|j| 2 + (j / k) * k).collect(),
            ]
            .concat(),
        )
    }
}
// @code end
