// Created by Ayush Biswas at 2025/06/30 16:00
// https://codeforces.com/problemset/problem/199/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
    ) -> Words<usize> {
        if n == 0 {
            return vec![0, 0, 0].into()
        }
        let (mut a, mut b) = (0, 1);
        while a + b != n {
            let c = a + b;
            a = b;
            b = c;
        }
        vec![0, a, b].into()
    }
}

// @code end
