// Created by Ayush Biswas at 2025/07/08 11:12
// https://cses.fi/problemset/task/1068/
use cp_lib::*;

// @code begin
use cpio::*;
use std::{
    iter::once,
    ops::{AddAssign, DivAssign, MulAssign},
};

sol! {
    fn solution(
        n: usize,
    ) -> Words<usize> {
        let res = (1..).scan(n, |state, _| {
            if *state == 1 {
                return None;
            }

            if *state%2 == 0 {
                state.div_assign(2);
            } else {
                state.mul_assign(3);
                state.add_assign(1);
            }
            Some(*state)
        });

        once(n).chain(res).collect()
    }
}

// @code end
