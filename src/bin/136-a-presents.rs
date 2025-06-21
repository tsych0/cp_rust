// Created by Ayush Biswas at 2025/05/20 11:24
// https://codeforces.com/problemset/problem/136/A
use cp_rust::*;

// @code begin
use cpio::*;
#[allow(unused)]
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> Words<usize>
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let p: Vec<usize> = input.read_line(parse_vec).unwrap();
    let mut res = vec![0; n];
    for i in 0..n {
        res[p[i] - 1] = i + 1;
    }
    res.into()
}
// @code end
