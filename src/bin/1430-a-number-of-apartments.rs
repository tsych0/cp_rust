// Created by Ayush Biswas at 2025/06/26 22:35
// https://codeforces.com/problemset/problem/1430/A
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        n: isize,
    ) -> Words<isize> {
        for i in 0.. {
            let threes = 3 * i;
            if threes == n {
                return vec![i, 0, 0].into();
            }
            if threes > n {
                break;
            }
            for j in 0.. {
                let fives = 5 * j;
                if threes + fives == n {
                    return vec![i, j, 0].into();
                }
                if threes + fives > n {
                    break;
                }
                for k in 0.. {
                    let sevens = 7 * k;
                    if threes + fives + sevens == n {
                        return vec![i, j, k].into();
                    }
                    if threes + fives + sevens > n {
                        break;
                    }
                }
            }
        }
        vec![-1].into()
    }
}

// @code end
