// Created by Ayush Biswas at 2025/06/03 20:14
// https://codeforces.com/problemset/problem/1917/A

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    (
        n is usize,
        a is [isize]
    ) -> String
    {
        if a.iter().filter(|&&ai| ai < 0).count() % 2 == 0
            && a.iter().filter(|&&ai| ai == 0).count() == 0
        {
            "1\n1 0".into()
        } else {
            "0".into()
        }
    }
}

// @code end
