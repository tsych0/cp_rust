// Created by Ayush Biswas at 2025/06/12 16:24
// https://codeforces.com/problemset/problem/1927/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> String
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let a: Vec<usize> = input.read_line(parse_vec).unwrap();
    let mut trace = HashMap::<usize, VecDeque<char>>::new();
    trace.insert(
        0,
        VecDeque::from(('a'..='z').into_iter().collect::<Vec<_>>()),
    );

    let mut res = vec![];
    for ai in a {
        let ti = trace.get_mut(&ai).unwrap();
        let c = ti.pop_front().unwrap();
        res.push(c);
        if trace.contains_key(&(ai + 1)) {
            let ti = trace.get_mut(&(ai + 1)).unwrap();
            ti.push_back(c);
        } else {
            trace.insert(ai + 1, VecDeque::from(vec![c]));
        }
    }

    res.into_iter().collect()
}
// @code end
