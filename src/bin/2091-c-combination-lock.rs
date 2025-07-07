// Created by Ayush Biswas at 2025/07/02 20:27
// https://codeforces.com/problemset/problem/2091/C
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        n: isize,
    ) -> Words<isize> {
        let mut res = vec![0; n as usize];
        for i in 1..=n {
            let idx = ((2*i - 2)%n) as usize;
            if res[idx] != 0 {
                return vec![-1].into();
            }
            res[idx] = i;
        }
        res.into()
    }
}

// @code end
