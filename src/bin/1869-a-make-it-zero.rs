// Created by Ayush Biswas at 2025/06/12 23:59
// https://codeforces.com/problemset/problem/1869/A

use cp_lib::*;

// @code begin
use cpio::*;
use std::io::Read;

sol! {
    (
        n is usize,
        _a is [usize]
    ) -> Lines<String>
    {
        vec![
             "4".into(),
             format!("1 {n}"),
             format!("{} {n}", 1 + n % 2),
             format!("1 2"),
             format!("1 2"),
         ]
         .into()
    }
}

// @code end
