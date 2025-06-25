// Created by Ayush Biswas at 2025/06/16 11:26
// https://codeforces.com/problemset/problem/1878/C

use cp_lib::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

sol! {
    (
        [n, k, x] is [usize; 3]
    ) -> bool
    {
        let l = n - k;
          let full_sum = (n * (n + 1)) / 2;
          let half_sum = (k * (k + 1)) / 2;
          let quater_sum = (l * (l + 1)) / 2;

          x >= half_sum && x <= (full_sum - quater_sum)
    }
}

// @code end
