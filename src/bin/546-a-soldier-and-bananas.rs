// Created by Ayush Biswas at 2025/05/17 09:40
// https://codeforces.com/problemset/problem/546/A
use cf_rust::cpio;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> isize
where
    R: Read,
{
    let [k, n, w]: [isize; 3] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    (k * ((w * (w + 1)) / 2) - n).max(0)
}
// @code end
