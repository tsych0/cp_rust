// Created by Ayush Biswas at 2025/06/03 20:14
// https://codeforces.com/problemset/problem/1917/A

use cp_lib::*;

// @code begin
use cpio::*;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> String
where
    R: Read,
{
    let _n: usize = input.read_line(parse).unwrap();
    let a: Vec<isize> = input.read_line(parse_vec).unwrap();
    if a.iter().filter(|&&ai| ai < 0).count() % 2 == 0
        && a.iter().filter(|&&ai| ai == 0).count() == 0
    {
        "1\n1 0".into()
    } else {
        "0".into()
    }
}
// @code end
