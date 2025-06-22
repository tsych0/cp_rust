// Created by Ayush Biswas at 2025/06/14 12:37
// https://codeforces.com/problemset/problem/1758/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
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
    let n: usize = input.read_line(parse).unwrap();
    if n % 2 == 0 {
        ListOf(vec![vec![1, 3], vec![2; n - 2]].concat())
    } else {
        ListOf(vec![69; n])
    }
}
// @code end
