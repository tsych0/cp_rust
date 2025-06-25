// Created by Ayush Biswas at 2025/06/03 16:39
// https://codeforces.com/problemset/problem/1922/A

use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        n: usize,
        a: String,
        b: String,
        c: String,
    ) -> bool {
        let mut a = a.chars().into_iter();
        let mut b = b.chars().into_iter();
        let mut c = c.chars().into_iter();
        for _ in 0..n {
            let ai = a.next().unwrap();
            let bi = b.next().unwrap();
            let ci = c.next().unwrap();

            if ai != ci && bi != ci {
                return true;
            }
        }
        false
    }
}

// @code end
