// Created by Ayush Biswas at 2025/05/14 10:41
// https://codeforces.com/problemset/problem/339/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    (        addition is String
    ) -> String
    {
        let mut nums = addition.split('+').collect::<Vec<_>>();
        nums.sort();
        nums.join("+")
    }
}

// @code end
