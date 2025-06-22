// Created by Ayush Biswas at 2025/05/14 20:12
// https://codeforces.com/problemset/problem/2056/A
use cp_lib::*;

// @code begin
use cpio::solve_n;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut cpio::CPInput<R>) -> u16
where
    R: Read,
{
    let [n, m]: [u16; 2] = input
        .read_line(cpio::parse_vec)
        .unwrap()
        .try_into()
        .unwrap();
    let xys: Vec<Vec<u16>> = input.read_lines(n as usize, cpio::parse_vec).unwrap();

    (4 * m)
        + xys
            .into_iter()
            .skip(1)
            .map(|xy| (xy[0] + xy[1]) * 2)
            .sum::<u16>()
}
// @code end
