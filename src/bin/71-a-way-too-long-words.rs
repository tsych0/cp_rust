// Created by Ayush Biswas at 2025/05/13 23:28
// https://codeforces.com/problemset/problem/71/A
use cp_lib::*;

// @code begin
use cpio::solve_n;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(io: &mut cpio::CPInput<R>) -> String
where
    R: Read,
{
    let s: String = io.read_line(cpio::parse).unwrap();
    let len = s.len();
    let mut s = s.chars();
    if len > 10 {
        format!("{}{}{}", s.next().unwrap(), len - 2, s.last().unwrap())
    } else {
        s.collect()
    }
}

// @code end
