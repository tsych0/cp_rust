// Created by Ayush Biswas at 2025/05/14 20:12
// https://codeforces.com/problemset/problem/2056/A
use cp_lib::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

sol! {
    (
        [n, m] is [u16; 2],
        xys is [[u16]]; n as usize
    ) -> u16
    {
        (4 * m)
            + xys
                .into_iter()
                .skip(1)
                .map(|xy| (xy[0] + xy[1]) * 2)
                .sum::<u16>()
    }
}

// @code end
