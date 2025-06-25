// Created by Ayush Biswas at 2025/06/14 19:30
// https://codeforces.com/problemset/problem/1742/C

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        _s: String,
        s: [[char]]; 8
    ) -> char {
        'l: for i in 0..8 {
            for j in 0..8 {
                if s[i][j] != 'R' {
                    continue 'l;
                }
            }
            return 'R';
        }
        'l: for i in 0..8 {
            for j in 0..8 {
                if s[j][i] != 'B' {
                    continue 'l;
                }
            }
            return 'B';
        }

        '.'
    }
}

// @code end
