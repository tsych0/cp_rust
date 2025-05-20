// Created by Ayush Biswas at 2025/05/16 17:04
// https://codeforces.com/problemset/problem/977/A
use cf_rust::cpio;

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
    let [mut n, k]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    for _ in 0..k {
        if n % 10 == 0 {
            n /= 10;
        } else {
            n -= 1;
        }
    }
    n
}
// @code end
