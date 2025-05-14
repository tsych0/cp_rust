// Created by Ayush Biswas at 2025/05/14 10:35
// https://codeforces.com/problemset/problem/2084/A
use cf_rust::cpio;

// @code begin
use cpio::{solve_n, ListOf};
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(io: &mut cpio::CPInput<R>) -> ListOf<i16>
where
    R: Read,
{
    let n: i16 = io.read_line(cpio::parse).unwrap();
    if n % 2 == 0 {
        ListOf::WordsOf(vec![-1])
    } else {
        let mut res = vec![n];
        res.extend_from_slice(&(1..n).collect::<Vec<_>>());
        ListOf::WordsOf(res)
    }
}
// @code end
