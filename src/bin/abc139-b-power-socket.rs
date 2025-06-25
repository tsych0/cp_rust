// Created by Ayush Biswas at 2025/06/23 16:34
// https://atcoder.jp/contests/abc139/tasks/abc139_b
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        [a, b]: [usize; 2]
    ) -> usize {
        if b < 2 {
            return 0;
        }
        1 + (b - 2) / (a - 1)
    }
}
// @code end
