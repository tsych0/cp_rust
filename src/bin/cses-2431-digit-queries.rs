// Created by Ayush Biswas at 2025/07/13 10:50
// https://cses.fi/problemset/task/2431
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        (mut k): usize
    ) -> usize {
        let mut mask = 9;
        let mut digits = 1;
        let mut offset = 0;
        while k >= mask * digits {
            k -= mask * digits;
            offset += mask;
            mask *= 10;
            digits += 1;
        }
        if k == 0 {
            return offset%10;
        }
        let n = offset + k.div_ceil(digits);
        if k%digits == 0 {
            n%10
        } else {
            let i = digits - k % digits;
            (n / 10usize.pow(i as u32)) % 10
        }
    }
}

// @code end
