// Created by Ayush Biswas at 2025/07/11 17:09
// https://cses.fi/problemset/task/3399
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        [n, a, b]: [usize; 3]
    ) -> CPResult<Lines<String>, String> {
        if a + b > n {
            return Err("NO".into()).into()
        }
        if a * b == 0 && a + b != 0 {
            return Err("NO".into()).into()
        }
        let m = a + b;
        Ok(vec![
            "YES".into(),
            (1..=n).collect::<Words<_>>().to_string(),
            (m - b + 1..=m).chain(1..=m-b).chain(m+1..=n).collect::<Words<_>>().to_string()
        ].into()).into()
    }
}

// @code end
