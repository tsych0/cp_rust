// Created by Ayush Biswas at 2025/06/07 14:46
// https://codeforces.com/problemset/problem/2102/A

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> bool
where
    R: Read,
{
    let [n, m, p, q]: [usize; 4] = input.read_line(parse_vec).unwrap().try_into().unwrap();

    if n % p == 0 && q * (n / p) == m {
        true
    } else {
        n % p != 0
    }
}
// @code end
