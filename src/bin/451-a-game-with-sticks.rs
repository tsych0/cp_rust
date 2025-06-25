// Created by Ayush Biswas at 2025/06/12 23:49
// https://codeforces.com/problemset/problem/451/A

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        [n, m]: [usize; 2]
    ) -> String {
        if n.min(m) % 2 == 0 {
            "Malvika"
        } else {
            "Akshat"
        }
        .into()
    }
}

// @code end
