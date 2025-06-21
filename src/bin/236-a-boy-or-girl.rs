// Created by Ayush Biswas at 2025/05/14 10:43
// https://codeforces.com/problemset/problem/236/A
use cp_rust::*;

// @code begin
use cpio::solve;
use std::{collections::HashSet, io::Read};

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut cpio::CPInput<R>) -> String
where
    R: Read,
{
    let s: String = input.read_line(cpio::parse).unwrap();
    if s.chars().collect::<HashSet<_>>().len() % 2 == 0 {
        "CHAT WITH HER!".into()
    } else {
        "IGNORE HIM!".into()
    }
}
// @code end
