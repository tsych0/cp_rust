// Created by Ayush Biswas at 2025/06/03 21:16
// https://codeforces.com/problemset/problem/1916/A

use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        [_n, k]: [usize; 2],
        b: [usize]
        ) -> String {
        let b_prod = b.iter().fold(1, |acc, bi| acc * bi);
        if 2023 % b_prod == 0 {
            format!(
                "YES\n{}",
                words_of([vec![1; k - 1].as_slice(), &[2023 / b_prod]].concat()).to_string()
            )
        } else {
            "NO".into()
        }
    }
}

// @code end
