// Created by Ayush Biswas at 2025/05/19 22:46
// https://codeforces.com/problemset/problem/1992/A
use cp_lib::*;

// @code begin
use cpio::*;
use std::io::Read;

sol! {
    (
        nums is [usize]
    ) -> usize
    {
        let mut nums = nums;
        for _ in 0..5 {
            let mut min_idx = 0;
            for j in 0..nums.len() {
                if nums[j] < nums[min_idx] {
                    min_idx = j;
                }
            }
            nums[min_idx] += 1;
        }
        nums.into_iter().product()
    }
}

// @code end
