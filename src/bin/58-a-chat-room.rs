// Created by Ayush Biswas at 2025/08/24 10:47
// https://codeforces.com/problemset/problem/58/A
use cp_lib::*;

// @code begin
use cpio::*;
// const INF: u32 = 10u32.pow(9);
// const INF: usize = 4 * 10usize.pow(18);

sol! {
    fn solution(
        s: String
    ) -> BOOL {
        let mut it = s.chars();
        for c in "hello".chars() {
            let found = it.find(|&i| i == c).is_some();
            if !found {
                return false.into()
            }
        } 
        true.into()
    }
}

// @code end
