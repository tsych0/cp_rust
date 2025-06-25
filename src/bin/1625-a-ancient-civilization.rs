// Created by Ayush Biswas at 2025/06/03 15:42
// https://codeforces.com/problemset/problem/1625/A

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    (
        [n, l] is [usize; 2],
        x is [usize]
    ) -> usize
    {
        let mut x_bin: Vec<Vec<u8>> = vec![];
           for xi in x {
               let mut xi_bin = vec![0; l];
               for i in 0..l {
                   xi_bin[i] = (((1 << i) & xi) >> i) as u8;
               }
               x_bin.push(xi_bin);
           }

           let mut res_bin = vec![0; l];
           for i in 0..l {
               let mut zero_count = 0;
               let mut one_count = 0;
               for j in 0..n {
                   if x_bin[j][i] == 1 {
                       one_count += 1;
                   } else {
                       zero_count += 1;
                   }
               }

               if one_count > zero_count {
                   res_bin[i] = 1;
               } else {
                   res_bin[i] = 0;
               }
           }

           res_bin.iter().rev().fold(0, |acc, yi| acc * 2 + yi)
    }
}

// @code end
