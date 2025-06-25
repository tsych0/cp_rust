// Created by Ayush Biswas at 2025/06/07 14:46
// https://codeforces.com/problemset/problem/2102/A

use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        [n, m, p, q]: [usize; 4]
    ) -> bool {
        if n % p == 0 && q * (n / p) == m {
            true
        } else {
            n % p != 0
        }
    }
}
// @code end
