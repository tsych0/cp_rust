// Created by Ayush Biswas at 2025/06/30 15:37
// https://codeforces.com/problemset/problem/1624/B
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        [a, b, c]: [usize; 3]
    ) -> bool {
        let i = b/2-a;
        let j = b/2-c;
        (2*(a+c))%b == 0 ||
        (b%2 == 0 && ((i%c == 0 && i*c > 0) || (j*a > 0 && j%a == 0)))
    }
}

// @code end
