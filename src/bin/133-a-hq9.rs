// Created by Ayush Biswas at 2025/06/08 12:51
// https://codeforces.com/problemset/problem/133/A

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    (
        s is String
    ) -> bool
    {
        s.contains(&['H', 'Q', '9'])
    }
}
// @code end
