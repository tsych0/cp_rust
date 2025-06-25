// Created by Ayush Biswas at 2025/05/20 09:07
// https://codeforces.com/problemset/problem/1984/A
use cp_lib::*;

// @code begin
use cpio::*;
use std::collections::HashSet;

sol_n! {
    fn solution(
        n: usize,
        a: [usize]
    ) -> String {
        let last_nums = a.get(1..n).unwrap().iter().collect::<HashSet<_>>();
        if last_nums.len() != 1 {
            "YES\n".to_string() + "R" + &vec!["B"; n - 1].into_iter().collect::<String>()
        } else if a[0] != a[1] {
            "YES\n".to_string() + "BR" + &vec!["B"; n - 2].into_iter().collect::<String>()
        } else {
            "NO".into()
        }
    }
}

// @code end
