// Created by Ayush Biswas at 2025/07/24 19:36
// https://cses.fi/problemset/task/1653
use cp_lib::*;
// @code begin
use cpio::*;

sol! {
    fn solution(
        [n, x]: [usize; 2],
        a: [usize]
    ) -> usize {
        let mut res = vec![(0, 0); 1 << n];
        res[0] = (1, 0);

        // Basically har subset mein apan kya kar rhe
        // kisi ek admi ko hata ke dekh rhe
        // ab ek ko hataenge to number kam hi banega
        // to wo pehle se calculated hi hoga na
        // ban gai dp
        for s in 1..1<<n {
            res[s] = (n + 1, 0);
            for p in 0..n {
                if (s & (1<<p)) != 0 { // pth person is present
                    let mut option = res[s ^ (1<<p)];
                    if option.1 + a[p] <= x { // we can take this person
                        option.1 += a[p];
                    } else {
                        option.1 = a[p];
                        option.0 += 1;
                    }
                    res[s] = res[s].min(option);
                }
            }
        }
        res[(1 << n) - 1].0
    }
}

// @code end
