// Created by Ayush Biswas at 2025/06/07 21:30
// https://codeforces.com/problemset/problem/1807/D

use cp_lib::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> Lines<bool>
where
    R: Read,
{
    let [_n, q]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let a: Vec<usize> = input.read_line(parse_vec).unwrap();
    let (a_partial_sum, a_sum) = a.into_iter().fold((vec![0], 0), |(mut acc, acc_sum), i| {
        acc.push(acc_sum + i);
        (acc, acc_sum + i)
    });
    let queries: Vec<Vec<usize>> = input.read_lines(q, parse_vec).unwrap();
    queries
        .into_iter()
        .map(|lrk| {
            let [l, r, k] = lrk.try_into().unwrap();
            let final_sum = a_sum - (a_partial_sum[r] - a_partial_sum[l - 1]) + (r - l + 1) * k;
            final_sum % 2 == 1
        })
        .collect()
}
// @code end
