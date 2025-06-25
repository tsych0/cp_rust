// Created by Ayush Biswas at 2025/05/14 10:27
// https://codeforces.com/problemset/problem/2094/B
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    (
        [n, m, mut l, mut r] is [isize; 4]
    ) -> Words<isize>
    {
        if m > n {
            r += m - n;
        } else {
            for _ in 0..(n - m) {
                if r > 0 {
                    r -= 1;
                } else {
                    l += 1;
                }
            }
        }

        ListOf(vec![l, r])
    }
}
// @code end
