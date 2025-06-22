// Created by Ayush Biswas at 2025/06/18 15:12
// https://codeforces.com/problemset/problem/1612/B

use cp_lib::*;

use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> Words<isize>
where
    R: Read,
{
    let [n, a, b]: [usize; 3] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let mut res = vec![0; n];
    res[0] = a;
    res[n - 1] = b;
    for (i, c) in (1..=n).filter(|&x| x != a && x != b).rev().enumerate() {
        res[i + 1] = c;
    }
    if res[0..n / 2].into_iter().min().unwrap() == &a
        && res[n / 2..n].into_iter().max().unwrap() == &b
    {
        ListOf(res.into_iter().map(|x| x as isize).collect())
    } else {
        ListOf(vec![-1])
    }
}
// @code end
