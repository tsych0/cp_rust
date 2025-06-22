// Created by Ayush Biswas at 2025/06/19 11:16
// https://codeforces.com/problemset/problem/1496/A

use cp_lib::*;

// @code begin
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
    let [n, k]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let s: String = input.read_line(parse).unwrap();
    let s_rev = s.chars().rev().collect::<String>();
    n > 2 * k && s[..k] == s_rev[..k] && s[n - k..] == s_rev[n - k..]
}
// @code end
