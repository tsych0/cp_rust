// Created by Ayush Biswas at 2025/06/03 16:39
// https://codeforces.com/problemset/problem/1922/A

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
    let n: usize = input.read_line(parse).unwrap();
    let a: String = input.read_line(parse).unwrap();
    let b: String = input.read_line(parse).unwrap();
    let c: String = input.read_line(parse).unwrap();
    let mut a = a.chars().into_iter();
    let mut b = b.chars().into_iter();
    let mut c = c.chars().into_iter();
    for i in 0..n {
        let ai = a.next().unwrap();
        let bi = b.next().unwrap();
        let ci = c.next().unwrap();

        if ai != ci && bi != ci {
            return true;
        }
    }
    false
}
// @code end
