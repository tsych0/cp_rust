// Created by Ayush Biswas at 2025/06/06 15:53
// https://codeforces.com/problemset/problem/1886/A
#![allow(unused)]

use cf_rust::cpio;
use cf_rust::itertools;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> Words<String>
where
    R: Read,
{
    let n: isize = input.read_line(parse).unwrap();

    for [i, j, k] in [[1, 1, 1], [2, 2, 2], [1, 2, 2], [2, 1, 1]] {
        let rem_n = n - (i + j + k);
        if rem_n % 3 != 0 {
            continue;
        }
        let sum_ijk = rem_n / 3;
        if i == j {
            if sum_ijk >= 3 {
                return vec![
                    "YES".into(),
                    words_of(vec![i, 3 + j, (sum_ijk - 1) * 3 + k]).to_string(),
                ]
                .into();
            }
        } else {
            if sum_ijk >= 1 {
                return vec![
                    "YES".into(),
                    words_of(vec![i, j, sum_ijk * 3 + k]).to_string(),
                ]
                .into();
            }
        }
    }
    ListOf(vec!["NO".into()])
}
// @code end
