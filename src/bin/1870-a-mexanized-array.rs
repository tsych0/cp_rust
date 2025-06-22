// Created by Ayush Biswas at 2025/06/07 13:19
// https://codeforces.com/problemset/problem/1870/A

use cp_lib::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> isize
where
    R: Read,
{
    let [n, k, x]: [isize; 3] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    if k > n || k > x + 1 {
        return -1;
    }
    if x == k {
        (0..k).sum::<isize>() + (n - k) * (x - 1)
    } else {
        (0..k).sum::<isize>() + (n - k) * x
    }
}
// @code end
