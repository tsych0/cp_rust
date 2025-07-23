// Created by Ayush Biswas at 2025/07/09 22:34
// https://cses.fi/problemset/task/1070
use cp_lib::*;

// @code begin
use cpio::*;
use std::iter::once;

sol! {
    fn solution(
        n: usize,
    ) -> Result<Words<usize>> {
        if n == 1 {
            return Ok(once(1).collect())
        }
        if n <= 3 {
            return Err("NO SOLUTION".into())
        }

        Ok((2..=n).step_by(2).chain((1..=n).step_by(2)).collect())
    }
}

// @code end
