// Created by Ayush Biswas at 2025/05/13 23:21
// https://codeforces.com/problemset/problem/2096/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    (
        n is usize,
        a is String
    ) -> Words<usize>
    {
        let mut res = vec![];
        let (mut front, mut back) = (1, n);
        for a_i in a.trim().chars().rev() {
            if a_i == '>' {
                res.push(back);
                back -= 1;
            } else {
                res.push(front);
                front += 1;
            }
        }
        res.push(front);
        res.reverse();
        ListOf(res)
    }
}
// @code end
