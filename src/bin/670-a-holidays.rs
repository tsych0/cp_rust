// Created by Ayush Biswas at 2025/06/30 14:12
// https://codeforces.com/problemset/problem/670/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
    ) -> Words<usize> {
        let weekly_holidays = (n/7) * 2;
        let extra_holidays = n%7;
        vec![weekly_holidays + extra_holidays.saturating_sub(5), weekly_holidays + extra_holidays.min(2)].into()
    }
}

// @code end
