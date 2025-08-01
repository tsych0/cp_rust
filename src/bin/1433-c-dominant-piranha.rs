// Created by Ayush Biswas at 2025/06/26 16:35
// https://codeforces.com/problemset/problem/1433/C
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        n: usize,
        a: [usize]
    ) -> CPResult<usize, i8> {
        let am = *a.iter().max().unwrap();
        for (i, _) in a.iter().enumerate().filter(|&(_, &ai)| ai == am) {
            if i > 0 && a[i-1] < am {
                return Success(1+i);
            }
            if i < n - 1 && a[i+1] < am {
                return Success(1+i);
            }
        }
        Failure(-1)
    }
}

// @code end
