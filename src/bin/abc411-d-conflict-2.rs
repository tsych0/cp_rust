// Created by Ayush Biswas at 2025/06/21 18:13
// https://atcoder.jp/contests/abc411/tasks/abc411_d

use cp_lib::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> String
where
    R: Read,
{
    let [_n, q]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();

    let queries: Vec<Vec<String>> = input.read_lines(q, parse_vec).unwrap();
    let mut res = vec![];
    let mut current_pc: usize = 0;

    for q in queries.into_iter().rev() {
        if q[0] == "3" && current_pc == 0 {
            current_pc = q[1].parse().unwrap();
        } else if q[1].parse::<usize>().unwrap() == current_pc {
            if q[0] == "2" {
                res.push(q.into_iter().nth(2).unwrap());
            } else if q[0] == "1" {
                current_pc = 0;
            }
        }
    }

    res.reverse();
    res.join("")
}
// @code end
