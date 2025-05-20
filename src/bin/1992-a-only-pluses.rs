// Created by Ayush Biswas at 2025/05/19 22:46
// https://codeforces.com/problemset/problem/1992/A
use cf_rust::cpio;

// @code begin
use cpio::*;
#[allow(unused)]
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let mut nums: Vec<usize> = input.read_line(parse_vec).unwrap();
    for _ in 0..5 {
        let mut min_idx = 0;
        for j in 0..nums.len() {
            if nums[j] < nums[min_idx] {
                min_idx = j;
            }
        }
        nums[min_idx] += 1;
    }
    nums.into_iter().product()
}
// @code end
