// Created by Ayush Biswas at 2025/06/06 17:10
// https://codeforces.com/problemset/problem/467/A

use cp_lib::*;

// @code begin
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
    let pq: Vec<Vec<usize>> = input.read_lines(n, parse_vec).unwrap();
    pq.into_iter().filter(|pqi| pqi[1] - pqi[0] >= 2).count()
}
// @code end
