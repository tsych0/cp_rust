// Created by Ayush Biswas at 2025/06/21 13:56
// https://codeforces.com/problemset/problem/1881/B

use cp_lib::*;

// @code begin
use cpio::*;
use math::*;

sol_n! {
    fn solution(
        [a, b, c]: [usize; 3]
    ) -> bool {
        let d = gcd(gcd(a, b), gcd(b, c));
        (a/d) + (b/d) + (c/d) - 3 <= 3
    }
}
// @code end
