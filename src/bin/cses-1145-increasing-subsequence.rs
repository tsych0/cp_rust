// Created by Ayush Biswas at 2025/07/23 18:15
// https://cses.fi/problemset/task/1145
use cp_lib::*;

// @code begin
use cpio::*;

const INF: usize = 10usize.pow(9) + 7;

sol! {
    fn solution(
        n: usize,
        a: [usize]
    ) -> usize {
        let mut lis = vec![];

        for ai in a {
            let point = lis.partition_point(|&l| l < ai);
            if point == lis.len() {
                lis.push(ai);
            } else {
                lis[point] = ai;
            }
        }

        lis.len()
    }
}

// @code end
