// Created by Ayush Biswas at 2025/06/06 15:03
// https://codeforces.com/problemset/problem/1890/B
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

fn solution<R>(input: &mut CPInput<R>) -> bool
where
    R: Read,
{
    let [n, k]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let s: Vec<u8> = input.read_line(parse_binary).unwrap();
    let t: Vec<u8> = input.read_line(parse_binary).unwrap();
    if s.iter().group_by(|&c| c).filter(|g| g.len() > 1).count() == 0 {
        true
    } else {
        if t.iter().group_by(|&c| c).filter(|g| g.len() > 1).count() > 0
            || t.first().unwrap() != t.last().unwrap()
        {
            false
        } else {
            let inserter = t.first().unwrap();
            s.iter()
                .group_by(|&c| c)
                .filter(|g| g.len() > 1)
                .all(|g| g[0] != inserter)
        }
    }
}
// @code end
