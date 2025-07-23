// Created by Ayush Biswas at 2025/07/21 10:59
// https://cses.fi/problemset/task/2413
use cp_lib::*;

// @code begin
use cpio::*;
const MOD: usize = 1_000_000_007;

enum Extend {
    JoinedBoth = 0,
    JoinedNone = 1,
    SepBoth = 2,
    SepLeft = 3,
    SepRight = 4,
    SepNone = 5,
}

sol! {
    fn solution(
        t: usize,
        queries: [usize]; t
    ) -> Lines<usize> {
        let mut res = vec![vec![0; 6]; 10usize.pow(6) + 1];

        use Extend::*;
        res[1][JoinedNone as usize] = 1;
        res[1][SepNone as usize] = 1;

        for h in 2..=*queries.iter().max().unwrap() {
            res[h][JoinedBoth as usize] =
                res[h - 1][JoinedBoth as usize ..= JoinedNone as usize].iter().sum::<usize>() % MOD;
            res[h][JoinedNone as usize] = res[h - 1].iter().sum::<usize>() % MOD;

            let sep_value = res[h - 1][SepBoth as usize ..= SepNone as usize].iter().sum::<usize>() % MOD;
            res[h][SepBoth as usize ..= SepRight as usize].fill(sep_value);

            res[h][SepNone as usize] = res[h - 1].iter().sum::<usize>() % MOD;
        }

        queries.into_iter().map(|q| {
            res[q].iter().sum::<usize>() % MOD
        }).collect()
    }
}

// @code end
