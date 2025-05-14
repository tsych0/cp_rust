// Created by Ayush Biswas at 2025/05/14 10:30
// https://codeforces.com/problemset/problem/263/A
use cf_rust::cpio;

// @code begin
use cpio::solve;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(io: &mut cpio::CPInput<R>) -> i8
where
    R: Read,
{
    let matrix: Vec<Vec<u8>> = io.read_lines(5, cpio::parse_vec).unwrap();
    for i in 0..5 {
        for j in 0..5 {
            if matrix[i][j] == 1 {
                return (i as i8 - 2).abs() + (j as i8 - 2).abs();
            }
        }
    }

    0
}
// @code end
