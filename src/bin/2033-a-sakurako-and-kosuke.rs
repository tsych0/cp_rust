// Created by Ayush Biswas at 2025/05/15 12:54
// https://codeforces.com/problemset/problem/2033/A
use cp_lib::*;

// @code begin
use cpio::*;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> String
where
    R: Read,
{
    let n: i32 = input.read_line(parse).unwrap();
    let mut sum = 0;
    for i in 1.. {
        sum += (-1 as i32).pow(i as u32) * (2 * i - 1);
        if sum.abs() > n {
            return match i % 2 {
                0 => "Kosuke".into(),
                _ => "Sakurako".into(),
            };
        }
    }
    "".into()
}
// @code end
