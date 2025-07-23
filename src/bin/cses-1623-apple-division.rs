// Created by Ayush Biswas at 2025/07/10 20:44
// https://cses.fi/problemset/task/1623
use cp_lib::*;

// @code begin
use cpio::*;
use std::cmp::min;

sol! {
    fn solution(
        _n: usize,
        a: [usize]
    ) -> usize {
        let sum = a.iter().sum::<usize>();
        let d = knapsack(&a, 0, sum/2);
        let acheived_sum = sum/2-d;
        sum - 2*acheived_sum
    }
}

fn knapsack(a: &Vec<usize>, i: usize, target: usize) -> usize {
    if i >= a.len() {
        return target;
    }

    if a[i] > target {
        return knapsack(a, i + 1, target);
    }

    min(
        knapsack(a, i + 1, target - a[i]),
        knapsack(a, i + 1, target),
    )
}

// @code end
