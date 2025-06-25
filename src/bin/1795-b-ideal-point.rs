// Created by Ayush Biswas at 2025/06/14 16:36
// https://codeforces.com/problemset/problem/1795/B

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    (
        [n, k] is [usize; 2],
        lrs is [[usize]]; n
    ) -> bool
    {
        for lr in &lrs {
            if lr[0] == k && lr[0] == lr[1] {
                return true;
            }
        }

        for lr1 in &lrs {
            for lr2 in &lrs {
                if (lr1[0] == k && lr1[0] == lr2[1]) || (lr1[1] == k && lr1[1] == lr2[0]) {
                    return true;
                }
            }
        }

        false
    }
}

// @code end
