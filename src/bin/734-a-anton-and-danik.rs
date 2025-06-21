// Created by Ayush Biswas at 2025/05/19 16:28
// https://codeforces.com/problemset/problem/734/A
use cp_rust::*;

// @code begin
use cpio::*;
#[allow(unused)]
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> String
where
    R: Read,
{
    let _n: usize = input.read_line(parse).unwrap();
    let s: String = input.read_line(parse).unwrap();
    let anton = s.chars().filter(|&c| c == 'A').count();
    let danik = s.chars().filter(|&c| c == 'D').count();
    if anton > danik {
        "Anton"
    } else if anton < danik {
        "Danik"
    } else {
        "Friendship"
    }
    .into()
}
// @code end
