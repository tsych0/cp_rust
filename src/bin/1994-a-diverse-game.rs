// Created by Ayush Biswas at 2025/05/19 16:36
// https://codeforces.com/problemset/problem/1994/A
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
    let [n, m]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let mut matrix: Vec<Vec<isize>> = input.read_lines(n, parse_vec).unwrap();
    if n != 1 {
        let x = matrix.pop().unwrap();
        matrix.insert(0, x);
        matrix.into_iter().map(|v| ListOf(v)).collect()
    } else if m != 1 {
        for i in 0..n {
            let y = matrix[i].pop().unwrap();
            matrix[i].insert(0, y);
        }
        matrix.into_iter().map(|v| ListOf(v)).collect()
    } else {
        ListOf(vec![ListOf(vec![-1])])
    }
}
// @code end
