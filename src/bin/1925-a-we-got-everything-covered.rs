// Created by Ayush Biswas at 2025/06/03 16:26
// https://codeforces.com/problemset/problem/1925/A

use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        [n, k]: [usize; 2]
    ) -> String {
        vec![('a'..).take(k).collect::<String>(); n].concat()
    }
}

// @code end
