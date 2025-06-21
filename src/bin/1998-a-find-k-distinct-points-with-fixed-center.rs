// Created by Ayush Biswas at 2025/05/19 10:38
// https://codeforces.com/problemset/problem/1998/A
use cp_rust::*;

// @code begin
use cpio::*;
#[allow(unused)]
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> Lines<Words<isize>>
where
    R: Read,
{
    let [x, y, k]: [isize; 3] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let j = 100000;
    let mut res = (1..(k / 2) + 1)
        .map(|i| [vec![i * j, i * j], vec![-i * j, -i * j]])
        .collect::<Vec<_>>()
        .concat();
    if k % 2 == 0 {
        res[0] = vec![res[0][0] + x * k, res[0][1] + y * k];
    } else {
        res.push(vec![x * k, y * k]);
    }
    res.into_iter().map(ListOf).collect()
}
// @code end
