// Created by Ayush Biswas at 2025/05/14 10:35
// https://codeforces.com/problemset/problem/2084/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: i16
    ) -> Words<i16> {
        if n % 2 == 0 {
            ListOf(vec![-1])
        } else {
            (n..=n).chain(1..n).collect()
        }
    }
}
// @code end
