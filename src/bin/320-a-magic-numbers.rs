// Created by Ayush Biswas at 2025/06/27 09:46
// https://codeforces.com/problemset/problem/320/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: String,
    ) -> Bool {
        n.split("144")
            .map(|x| x.split("14")
                    .map(|y| y.split("1").collect::<String>())
                    .collect::<Vec<_>>().join(""))
            .collect::<Vec<_>>().join("").is_empty().into()
    }
}

// @code end
