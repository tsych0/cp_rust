// Created by Ayush Biswas at 2025/06/12 23:49
// https://codeforces.com/problemset/problem/451/A

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> String
where
    R: Read,
{
    let [n, m]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    if n.min(m) % 2 == 0 {
        "Malvika"
    } else {
        "Akshat"
    }
    .into()
}
// @code end
