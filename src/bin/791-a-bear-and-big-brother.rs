// Created by Ayush Biswas at 2025/05/15 13:32
// https://codeforces.com/problemset/problem/791/A
use cp_rust::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let [mut a, mut b]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    for i in 1.. {
        a *= 3;
        b *= 2;
        if a > b {
            return i;
        }
    }
    0
}
// @code end
