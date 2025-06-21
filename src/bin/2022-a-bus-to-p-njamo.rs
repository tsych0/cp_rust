// Created by Ayush Biswas at 2025/05/17 10:14
// https://codeforces.com/problemset/problem/2022/A
use cp_rust::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let [_n, r]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let a: Vec<usize> = input.read_line(parse_vec).unwrap();
    let rows_used: usize = a.iter().map(|ai| ai / 2).sum();
    let people_left: usize = a.iter().map(|ai| ai % 2).sum();
    let rows_left = r - rows_used;
    if people_left <= rows_left {
        rows_used * 2 + people_left
    } else {
        rows_used * 2 + rows_left - (people_left - rows_left)
    }
}
// @code end
