// Created by Ayush Biswas at 2025/05/14 10:45
// https://codeforces.com/problemset/problem/2082/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    (
        [n, m] is [usize; 2],
        matrix is [[01]]; n
    ) -> u16
    {
        u16::max(
            (0..n).fold(0, |acc, i| {
                (0..m).fold(0, |xor, j| xor ^ matrix[i][j]) as u16 + acc
            }), // count the number of rows that have non zero xor
            (0..m).fold(0, |acc, j| {
                (0..n).fold(0, |xor, i| xor ^ matrix[i][j]) as u16 + acc
            }), // count the number of columns that have non zero xor
        )
    }
}
// @code end
