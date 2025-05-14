// Created by Ayush Biswas at 2025/05/13 23:21
// https://codeforces.com/problemset/problem/2096/A
use cf_rust::cpio;

// @code begin
use cpio::{solve_n, ListOf};
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(io: &mut cpio::CPInput<R>) -> ListOf<usize>
where
    R: Read,
{
    let n = io.read_line(cpio::parse).unwrap();
    let a: String = io.read_line(cpio::parse).unwrap();
    let mut res = vec![];
    let (mut front, mut back) = (1, n);
    for a_i in a.trim().chars().rev() {
        if a_i == '>' {
            res.push(back);
            back -= 1;
        } else {
            res.push(front);
            front += 1;
        }
    }
    res.push(front);
    res.reverse();
    ListOf::WordsOf(res)
}

// @code end
