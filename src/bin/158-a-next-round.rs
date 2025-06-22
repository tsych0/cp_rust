// Created by Ayush Biswas at 2025/05/14 19:55
// https://codeforces.com/problemset/problem/158/A
use cp_lib::*;

// @code begin
use cpio::solve;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut cpio::CPInput<R>) -> u8
where
    R: Read,
{
    let [_n, k]: [u8; 2] = input
        .read_line(cpio::parse_vec)
        .unwrap()
        .try_into()
        .unwrap();
    let a: Vec<u8> = input.read_line(cpio::parse_vec).unwrap();
    let mut current_rank = 0;
    let mut prev = 0;
    for ai in a.into_iter() {
        if ai == 0 {
            break;
        }
        if ai == prev {
            current_rank += 1;
        } else {
            if current_rank >= k {
                break;
            }
            current_rank += 1;
        }
        prev = ai;
    }
    current_rank
}
// @code end
