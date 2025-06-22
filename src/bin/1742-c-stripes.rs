// Created by Ayush Biswas at 2025/06/14 19:30
// https://codeforces.com/problemset/problem/1742/C

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> char
where
    R: Read,
{
    let s: String = input.read_line(parse).unwrap();
    let s: Vec<Vec<char>> = input.read_lines(8, parse_chars).unwrap();
    'l: for i in 0..8 {
        for j in 0..8 {
            if s[i][j] != 'R' {
                continue 'l;
            }
        }
        return 'R';
    }
    'l: for i in 0..8 {
        for j in 0..8 {
            if s[j][i] != 'B' {
                continue 'l;
            }
        }
        return 'B';
    }

    '.'
}
// @code end
