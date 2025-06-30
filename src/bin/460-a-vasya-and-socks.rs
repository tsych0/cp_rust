// Created by Ayush Biswas at 2025/06/30 14:24
// https://codeforces.com/problemset/problem/460/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        [mut n, m]: [usize; 2]
    ) -> usize {
        let mut res = 0;
        let mut residue = 0;
        while n != 0 {
            res += n;
            (n, residue) = ((n+residue)/m, (n+residue)%m)
        }
        res
    }
}

// @code end
