// Created by Ayush Biswas at 2025/05/14 10:27
// https://codeforces.com/problemset/problem/2094/B
use cf_rust::cpio;

// @code begin
use cpio::{solve_n, ListOf};
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(io: &mut cpio::CPInput<R>) -> ListOf<isize>
where
    R: Read,
{
    let [n, m, mut l, mut r] = io.read_line(cpio::parse_vec).unwrap().try_into().unwrap();

    if m > n {
        r += m - n;
    } else {
        for _ in 0..(n - m) {
            if r > 0 {
                r -= 1;
            } else {
                l += 1;
            }
        }
    }

    ListOf::WordsOf(vec![l, r])
}
// @code end
