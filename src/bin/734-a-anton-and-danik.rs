// Created by Ayush Biswas at 2025/05/19 16:28
// https://codeforces.com/problemset/problem/734/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        n: usize,
        s: String
    ) -> String {
        let anton = s.chars().filter(|&c| c == 'A').count();
        let danik = s.chars().filter(|&c| c == 'D').count();
        if anton > danik {
            "Anton"
        } else if anton < danik {
            "Danik"
        } else {
            "Friendship"
        }
        .into()
    }
}

// @code end
