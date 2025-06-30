// Created by Ayush Biswas at 2025/06/30 16:19
// https://codeforces.com/problemset/problem/11/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        [n, d]: [usize; 2],
        (mut a): [usize]
    ) -> usize {
        let mut moves = 0;
        for i in 1..n {
            if a[i] <= a[i-1] {
                let diff = a[i-1] - a[i];
                a[i] += ((diff+d)/d)*d;
                moves += (diff+d)/d;
            }
        }
        moves
    }
}

// @code end
