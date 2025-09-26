// Created by Ayush Biswas at 2025/07/02 20:46
// https://codeforces.com/problemset/problem/2055/B
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;

sol_n! {
    fn solution(_: usize, a: [usize], b: [usize]) -> BOOL {
        let amin = a
            .iter()
            .zip(b.iter())
            .filter(|(ai, bi)| ai >= bi)
            .map(|(ai, bi)| ai - bi)
            .min()
            .unwrap_or(usize::MAX)
            .clone();
        let required_products = a
            .into_iter()
            .zip(b.into_iter())
            .filter(|(ai, bi)| ai < bi)
            .collect_vec();
        ((required_products.is_empty())
            || (required_products.len() == 1 && (required_products[0].1 - required_products[0].0 <= amin))).into()
    }
}

// @code end
