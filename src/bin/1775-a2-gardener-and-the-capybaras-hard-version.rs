// Created by Ayush Biswas at 2025/06/13 15:18
// https://codeforces.com/problemset/problem/1775/A2

use cp_lib::*;

// @code begin
use cpio::*;
use std::io::Read;

sol! {
    (
        s is [char]
    ) -> Words<String>
    {
        let n = s.len();
        // let a = s.chars().take_while(|&c| c == 'a').collect::<String>();

        if s[0] == s[n - 1] {
            ListOf(vec![
                s[0].to_string(),
                s[1..n - 1].into_iter().collect(),
                s[n - 1].to_string(),
            ])
        } else {
            if s[0] == 'a' && s[1] == 'b' {
                ListOf(vec![
                    s[0].into(),
                    s[1..n - 1].into_iter().collect(),
                    s[n - 1].into(),
                ])
            } else {
                ListOf(vec![s[0].into(), s[1].into(), s[2..].into_iter().collect()])
            }
        }
    }
}

// @code end
