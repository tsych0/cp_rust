// Created by Ayush Biswas at 2025/06/19 10:23
// https://codeforces.com/problemset/problem/1537/B

use cp_lib::*;

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
    let [n, m, _, _]: [usize; 4] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    ListOf(vec![1, 1, n, m])
}
// @code end
