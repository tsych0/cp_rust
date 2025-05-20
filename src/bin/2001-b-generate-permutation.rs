// Created by Ayush Biswas at 2025/05/19 09:17
// https://codeforces.com/problemset/problem/2001/B
use cf_rust::cpio;

// @code begin
use cpio::*;
#[allow(unused)]
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> ListOf<isize>
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    if n % 2 == 0 {
        ListOf::WordsOf(vec![-1])
    } else {
        let second_part = (1..=n as isize).take((n + 1) / 2).collect::<Vec<_>>();
        let first_part = (1..=n as isize).rev().take(n / 2).collect::<Vec<_>>();
        ListOf::WordsOf(
            [first_part.as_slice(), second_part.as_slice()]
                .concat()
                .to_vec(),
        )
    }
}
// @code end
