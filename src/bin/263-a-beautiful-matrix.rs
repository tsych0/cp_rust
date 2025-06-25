// Created by Ayush Biswas at 2025/05/14 10:30
// https://codeforces.com/problemset/problem/263/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        matrix: [[u8]]; 5
    ) -> i8 {
        for i in 0..5 {
            for j in 0..5 {
                if matrix[i][j] == 1 {
                    return (i as i8 - 2).abs() + (j as i8 - 2).abs();
                }
            }
        }

        0
    }
}

// @code end
