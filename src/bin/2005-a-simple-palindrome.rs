// Created by Ayush Biswas at 2025/06/12 12:12
// https://codeforces.com/problemset/problem/2005/A

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;

sol! {
    fn solution(
        n: usize
    ) -> String {
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        vowels.into_iter().cycle().take(n).sorted().collect()
    }
}

// @code end
