// Created by Ayush Biswas at 2025/06/14 16:36
// https://codeforces.com/problemset/problem/1795/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> bool
where
    R: Read,
{
    let [n, k]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let lrs: Vec<Vec<usize>> = input.read_lines(n, parse_vec).unwrap();

    for lr in &lrs {
        if lr[0] == k && lr[0] == lr[1] {
            return true;
        }
    }

    for lr1 in &lrs {
        for lr2 in &lrs {
            if (lr1[0] == k && lr1[0] == lr2[1]) || (lr1[1] == k && lr1[1] == lr2[0]) {
                return true;
            }
        }
    }

    false
}
// @code end
