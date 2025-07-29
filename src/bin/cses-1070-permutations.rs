// Created by Ayush Biswas at 2025/07/09 22:34
// https://cses.fi/problemset/task/1070
use cp_lib::*;

// @code begin
use cpio::*;
use std::iter::once;

sol! {
    fn solution(
        n: usize,
    ) -> CPResult<Words<usize>, String> {
        if n == 1 {
            return Success(once(1).collect())
        }
        if n <= 3 {
            return Failure("NO SOLUTION".into())
        }

        Success((2..=n).step_by(2).chain((1..=n).step_by(2)).collect())
    }
}

// @code end
