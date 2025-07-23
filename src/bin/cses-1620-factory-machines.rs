// Created by Ayush Biswas at 2025/07/18 11:58
// https://cses.fi/problemset/task/1620
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution([_n, t]: [usize; 2], k: [usize]) -> usize {
        let max_time_taking_machine = *k.iter().max().unwrap();
        let (mut lo, mut hi) = (0, max_time_taking_machine * t);
        while lo < hi {
            let mid = (lo + hi) / 2;
            let toys: usize = k.iter().map(|ki| mid / ki).sum();
            if toys < t {
                lo = mid + 1;
            } else if toys >= t {
                hi = mid;
            }
        }
        lo
    }
}

// @code end
