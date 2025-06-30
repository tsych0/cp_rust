// Created by Ayush Biswas at 2025/06/30 13:58
// https://codeforces.com/problemset/problem/1771/A
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        n: usize,
        a: [usize]
    ) -> usize {
        let amax = *a.iter().max().unwrap();
        let amin = *a.iter().min().unwrap();
        if amax == amin {
            return n * (n - 1)
        }
        a.iter().filter(|&&ai| ai == amax).count() *
        a.iter().filter(|&&ai| ai == amin).count() * 2
    }
}

// @code end
