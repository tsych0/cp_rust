// Created by Ayush Biswas at 2025/06/26 22:35
// https://codeforces.com/problemset/problem/1430/A
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        n: usize,
    ) -> CPResult<Words<usize>, i8> {
        for i in 0.. {
            let threes = 3 * i;
            if threes == n {
                return Success(vec![i, 0, 0].into());
            }
            if threes > n {
                break;
            }
            for j in 0.. {
                let fives = 5 * j;
                if threes + fives == n {
                    return Success(vec![i, j, 0].into());
                }
                if threes + fives > n {
                    break;
                }
                for k in 0.. {
                    let sevens = 7 * k;
                    if threes + fives + sevens == n {
                        return Success(vec![i, j, k].into());
                    }
                    if threes + fives + sevens > n {
                        break;
                    }
                }
            }
        }
        Failure(-1)
    }
}

// @code end
