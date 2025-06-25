// Created by Ayush Biswas at 2025/06/19 10:23
// https://codeforces.com/problemset/problem/1537/B

use cp_lib::*;

use cpio::*;
use std::convert::TryInto;
use std::io::Read;

sol! {
    (
        [n, m, _, _] is [usize; 4]
    ) -> Words<usize>
    {
        ListOf(vec![1, 1, n, m])
    }
}

// @code end
