// Created by Ayush Biswas at 2025/05/13 23:28
// https://codeforces.com/problemset/problem/71/A
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        s: String
    ) -> String {
        let len = s.len();
        let mut s = s.chars();
        if len > 10 {
            format!("{}{}{}", s.next().unwrap(), len - 2, s.last().unwrap())
        } else {
            s.collect()
        }
    }
}

// @code end
