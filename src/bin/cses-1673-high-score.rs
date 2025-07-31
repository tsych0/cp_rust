// Created by Ayush Biswas at 2025/07/30 16:12
// https://cses.fi/problemset/task/1673
use cp_lib::*;

// @code begin
use cpio::*;
// const INF: u32 = 10u32.pow(9);
const INF: isize = 10isize.pow(18);

sol! {
    fn solution(
        [n, m]: [usize; 2],
        tunnels: [[isize; 3]; m]
    ) -> CPResult<isize, i8> {
        let mut d = vec![INF; n + 1];
        let mut reachable = vec![false; n + 1];
        reachable[1] = true;
        d[1] = 0;
        
        for _ in 1..n {
            for &[a, b, _] in &tunnels {
                reachable[b as usize] |= reachable[a as usize];
            }
        }
        
        for _ in 1..n {
            for &[a, b, x] in &tunnels {
                if reachable[a as usize] {
                    d[b as usize] = d[b as usize].min(d[a as usize] - x);
                }
            }
        }
        
        for _ in 1..=n {
            for &[a, b, x] in &tunnels {
                if d[a as usize] - x < d[b as usize] 
                    && reachable[a as usize] && reachable[b as usize] {
                    d[b as usize] = -INF;        
                }
            }
        }
        
        if d[n] >= INF || d[n] <= -INF {
            Failure(-1)
        } else {
            Success(-d[n])
        }
    }
}

// @code end
