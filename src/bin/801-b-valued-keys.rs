// Created by Ayush Biswas at 2025/06/29 18:14
// https://codeforces.com/problemset/problem/801/B
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        x: [char],
        z: [char]
    ) -> String {
        let y = z.clone();
        for (xi, zi) in x.into_iter().zip(z.into_iter()) {
            if xi < zi {
                return "-1".into()
            }
        }
        y.into_iter().collect()
    }
}

// @code end
