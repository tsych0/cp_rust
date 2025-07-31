// Created by Ayush Biswas at 2025/07/31 20:46
// https://cses.fi/problemset/task/1197
use cp_lib::*;

// @code begin
use cpio::*;
// const INF: u32 = 10u32.pow(9);
const INF: isize = 4 * 10isize.pow(18);

sol! {
    fn solution(
        [n, m]: [usize; 2],
        edges: [[isize; 3]; m]
    ) -> CPResult<Lines<String>, String> {
        let mut d = vec![INF; n + 1];
        let mut parent = vec![0; n + 1];
        
        d[1] = 0;
        let mut x = 0;
        
        for _ in 1..=n {
            x = 0;
            for &[a, b, c] in &edges {
                if d[a as usize] + c < d[b as usize] {
                    d[b as usize] = d[a as usize] + c;
                    parent[b as usize] = a as usize;
                    x = b as usize;
                }
            }
        }
        
        if x != 0 {
            for _ in 0..n {
                x = parent[x];
            } 
            let mut cycle = vec![x];
            let y = x;
            loop {
                x = parent[x];
                cycle.push(x);
                if x == y {
                    break;
                } 
            }
            cycle.reverse();
            Success(vec![
                "YES".into(),
                words_of(cycle).to_string()
            ].into())
        } else {
            Failure("NO".into())
        }
    }
}

// @code end
