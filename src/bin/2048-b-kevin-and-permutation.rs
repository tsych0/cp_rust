// Created by Ayush Biswas at 2025/06/08 23:43
// https://codeforces.com/problemset/problem/2048/B

use cp_lib::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

sol! {
    (
        [n, k] is [usize; 2]
    ) -> Words<usize>
    {
        let mut res = vec![0; n];
        let mut taken = 1;
        for i in (1..=n).map(|j| k * j).take_while(|&j| j <= n) {
            res[i - 1] = taken;
            taken += 1;
        }

        res.iter_mut().for_each(|r| {
            if *r == 0 {
                *r = taken;
                taken += 1;
            }
        });

        ListOf(res)
    }
}

// @code end
