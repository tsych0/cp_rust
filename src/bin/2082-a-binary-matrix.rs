// Created by Ayush Biswas at 2025/05/14 10:45
// https://codeforces.com/problemset/problem/2082/A
use cf_rust::cpio;

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
    let [n, m]: [usize; 2] = input
        .read_line(cpio::parse_vec)
        .unwrap()
        .try_into()
        .unwrap();
    let matrix: Vec<Vec<u8>> = input.read_lines(n, cpio::parse_binary).unwrap();

    u16::max(
        (0..n).fold(0, |acc, i| {
            (0..m).fold(0, |xor, j| xor ^ matrix[i][j]) as u16 + acc
        }), // count the number of rows that have non zero xor
        (0..m).fold(0, |acc, j| {
            (0..n).fold(0, |xor, i| xor ^ matrix[i][j]) as u16 + acc
        }), // count the number of columns that have non zero xor
    )
}
// @code end
