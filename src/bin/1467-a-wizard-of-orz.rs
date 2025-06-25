// Created by Ayush Biswas at 2025/06/19 11:37
// https://codeforces.com/problemset/problem/1467/A

use cp_lib::*;

// @code begin
use cpio::*;
use std::io::Read;

sol! {
    fn (
        n: usize
    ) -> ListOf<'\0', usize> {
        vec![9]
              .into_iter()
              .chain((1..n).map(|i| (i + 7) % 10))
              .collect()
    }
}

// @code end
