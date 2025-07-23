// Created by Ayush Biswas at 2025/07/17 15:25
// https://cses.fi/problemset/task/2217
use cp_lib::*;

// @code begin
use cpio::*;
use std::collections::BTreeSet;

sol! {
    fn solution(
        [n, k]: [usize; 2],
        a: [usize],
        swaps: [[usize; 2]; k]
    ) -> Lines<usize> {
        let mut positions = vec![0; n + 1];
        a.iter().enumerate().for_each(|(i, &ai)| positions[ai] = i);
        let passes = 1 + (1..n).filter(|&i| positions[i] > positions[i+1]).count();

        swaps.into_iter().scan((a, positions, passes), |(a, positions, passes), [mut i, mut j]| {
            i -= 1; j -= 1;
            let ai = a[i];
            let aj = a[j];
            let mut check_points: BTreeSet<usize> = vec![ai, aj].into_iter().collect();
            if ai > 1 {
                check_points.insert(ai - 1);
            }
            if aj > 1 {
                check_points.insert(aj - 1);
            }
            for &i in &check_points {
                if i < n && positions[i] > positions[i+1] {
                    *passes -= 1;
                }
            }
            a.swap(i, j);
            positions.swap(ai, aj);

            for &i in &check_points {
                if i < n && positions[i] > positions[i+1] {
                    *passes += 1;
                }
            }
            Some(*passes)
        }).collect()
    }
}

// @code end
