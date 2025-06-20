// Created by Ayush Biswas at 2025/06/03 21:16
// https://codeforces.com/problemset/problem/1916/A
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

fn solution<R>(input: &mut CPInput<R>) -> String
where
    R: Read,
{
    let [n, k]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let b: Vec<usize> = input.read_line(parse_vec).unwrap();
    let b_prod = b.iter().fold(1, |acc, bi| acc * bi);
    if 2023 % b_prod == 0 {
        format!(
            "YES\n{}",
            words_of([vec![1; k - 1].as_slice(), &[2023 / b_prod]].concat()).to_string()
        )
    } else {
        "NO".into()
    }
}
// @code end
