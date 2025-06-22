// Created by Ayush Biswas at 2025/06/12 15:11
// https://codeforces.com/problemset/problem/337/A

use cp_lib::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;
use std::usize;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let [n, m]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let mut f: Vec<usize> = input.read_line(parse_vec).unwrap();
    f.sort();
    let mut res = usize::MAX;
    for i in 0..m - n + 1 {
        res = res.min(f[i + n - 1] - f[i])
    }
    res
}
// @code end
