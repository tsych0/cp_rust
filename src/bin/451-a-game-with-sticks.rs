// Created by Ayush Biswas at 2025/06/12 23:49
// https://codeforces.com/problemset/problem/451/A

use cp_lib::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

sol! {
    (        [n, m] is [usize; 2]
    ) -> String
    {
        if n.min(m) % 2 == 0 {
            "Malvika"
        } else {
            "Akshat"
        }
        .into()
    }
}

// @code end
