// Created by Ayush Biswas at 2025/07/02 19:59
// https://codeforces.com/problemset/problem/2092/B
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        n: usize,
        a: [01],
        b: [01]
    ) -> Bool {
        let a_f = a.iter().step_by(2);
        let a_s = a.iter().skip(1).step_by(2);
        let b_f = b.iter().step_by(2);
        let b_s = b.iter().skip(1).step_by(2);
        let f = a_f.chain(b_s).filter(|&&i| i == 0).count();
        let s = a_s.chain(b_f).filter(|&&i| i == 0).count();
        (f >= n.div_ceil(2) && s >= n / 2).into()
    }
}

// @code end
