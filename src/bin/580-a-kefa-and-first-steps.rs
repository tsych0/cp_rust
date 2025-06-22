// Created by Ayush Biswas at 2025/06/12 12:29
// https://codeforces.com/problemset/problem/580/A

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let a: Vec<usize> = input.read_line(parse_vec).unwrap();
    let mut r = vec![0; n];
    for i in 1..n {
        if a[i] < a[i - 1] {
            r[i] = r[i - 1] + 1;
        } else {
            r[i] = r[i - 1];
        }
    }
    r.into_iter()
        .group_by(|&i| i)
        .map(|g| g.len())
        .max()
        .unwrap()
}
// @code end
